// FFI (Foreign Function Interface) module for C# integration
// This module provides C-compatible interfaces for the COL runtime

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int};
use std::ptr;
use std::collections::HashMap;

use crate::codegen::ir_generator::IRGenerator;
use crate::codegen::jit::JITExecutor;
use crate::handler::parse_handler::ParseHandler;
use crate::parser::program::Program;
use inkwell::context::Context;

/// Opaque handle to a compiled COL script
pub struct COLScript {
    program: Program,
    global_variables: HashMap<String, COLVariant>,
    last_error: Option<String>,
}

/// Result codes for C# interop
#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum COLResult {
    Success = 0,
    ErrorCompilation = 1,
    ErrorExecution = 2,
    ErrorInvalidHandle = 3,
    ErrorInvalidParameter = 4,
}

/// C-compatible value type for cross-language communication
#[repr(C)]
#[derive(Copy, Clone)]
pub union COLValue {
    pub number: c_double,
    pub boolean: c_int,
    pub string_ptr: *mut c_char,
}

/// Tagged union for runtime values
#[repr(C)]
#[derive(Copy, Clone)]
pub struct COLVariant {
    pub value_type: c_int, // 0=number, 1=boolean, 2=string, 3=null
    pub value: COLValue,
}

/// Compile GML source code into a script handle
/// Returns null on failure
#[unsafe(no_mangle)]
pub extern "C" fn col_compile_script(source: *const c_char) -> *mut COLScript {
    if source.is_null() {
        return ptr::null_mut();
    }

    let source_str = match unsafe { CStr::from_ptr(source) }.to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let program = match ParseHandler::parse_source_code(source_str) {
        Ok(program) => program,
        Err(_) => return ptr::null_mut(),
    };

    Box::into_raw(Box::new(COLScript {
        program,
        global_variables: HashMap::new(),
        last_error: None,
    }))
}

/// Call a function in the compiled script
#[unsafe(no_mangle)]
pub extern "C" fn col_call_function(
    script: *mut COLScript,
    function_name: *const c_char,
    args: *const COLVariant,
    arg_count: c_int,
    result: *mut COLVariant,
) -> COLResult {
    if script.is_null() || function_name.is_null() {
        return COLResult::ErrorInvalidHandle;
    }

    let script = unsafe { &mut *script };
    let func_name = match unsafe { CStr::from_ptr(function_name) }.to_str() {
        Ok(s) => s,
        Err(_) => return COLResult::ErrorInvalidParameter,
    };

    // Create LLVM context for execution
    let context = Context::create();
    let mut ir_generator = IRGenerator::new(&context, "runtime_module");

    // Generate IR from the program
    match script.program.accept(&mut ir_generator) {
        Ok(_) => {
            // Verify the module
            if let Err(e) = ir_generator.get_module().verify() {
                script.last_error = Some(format!("Module verification failed: {}", e));
                return COLResult::ErrorCompilation;
            }

            // Create JIT executor
            match JITExecutor::new(ir_generator.get_module()) {
                Ok(executor) => {
                    // Convert arguments
                    let arg_values: Vec<f64> = if arg_count > 0 && !args.is_null() {
                        (0..arg_count)
                            .map(|i| {
                                let variant = unsafe { *args.offset(i as isize) };
                                match variant.value_type {
                                    0 => unsafe { variant.value.number },
                                    1 => if unsafe { variant.value.boolean } != 0 { 1.0 } else { 0.0 },
                                    _ => 0.0, // Default for other types
                                }
                            })
                            .collect()
                    } else {
                        Vec::new()
                    };

                    // Execute the function
                    match executor.execute_function(func_name, &arg_values) {
                        Ok(func_result) => {
                            if !result.is_null() {
                                unsafe {
                                    (*result).value_type = 0; // number type
                                    (*result).value.number = func_result;
                                }
                            }
                            COLResult::Success
                        }
                        Err(e) => {
                            script.last_error = Some(format!("Function execution failed: {}", e));
                            COLResult::ErrorExecution
                        }
                    }
                }
                Err(e) => {
                    script.last_error = Some(format!("JIT creation failed: {}", e));
                    COLResult::ErrorCompilation
                }
            }
        }
        Err(e) => {
            script.last_error = Some(format!("IR generation failed: {:?}", e));
            COLResult::ErrorCompilation
        }
    }
}

/// Set a global variable in the script
#[unsafe(no_mangle)]
pub extern "C" fn col_set_global_variable(
    script: *mut COLScript,
    var_name: *const c_char,
    value: *const COLVariant,
) -> COLResult {
    if script.is_null() || var_name.is_null() || value.is_null() {
        return COLResult::ErrorInvalidParameter;
    }

    let script = unsafe { &mut *script };
    let var_name_str = match unsafe { CStr::from_ptr(var_name) }.to_str() {
        Ok(s) => s,
        Err(_) => return COLResult::ErrorInvalidParameter,
    };

    let value_variant = unsafe { *value };
    script.global_variables.insert(var_name_str.to_string(), value_variant);
    
    COLResult::Success
}

/// Get a global variable from the script
#[unsafe(no_mangle)]
pub extern "C" fn col_get_global_variable(
    script: *mut COLScript,
    var_name: *const c_char,
    result: *mut COLVariant,
) -> COLResult {
    if script.is_null() || var_name.is_null() || result.is_null() {
        return COLResult::ErrorInvalidParameter;
    }

    let script = unsafe { &mut *script };
    let var_name_str = match unsafe { CStr::from_ptr(var_name) }.to_str() {
        Ok(s) => s,
        Err(_) => return COLResult::ErrorInvalidParameter,
    };

    match script.global_variables.get(var_name_str) {
        Some(value) => {
            unsafe {
                *result = *value;
            }
            COLResult::Success
        }
        None => {
            unsafe {
                (*result).value_type = 3; // null type
            }
            COLResult::Success
        }
    }
}

/// Free string memory allocated by COL runtime
#[unsafe(no_mangle)]
pub extern "C" fn col_free_string(string_ptr: *mut c_char) {
    if !string_ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(string_ptr);
        }
    }
}

/// Destroy a script handle and free its memory
#[unsafe(no_mangle)]
pub extern "C" fn col_destroy_script(script: *mut COLScript) {
    if !script.is_null() {
        unsafe {
            let _ = Box::from_raw(script);
        }
    }
}

/// Get the last error message (thread-local)
#[unsafe(no_mangle)]
pub extern "C" fn col_get_last_error() -> *const c_char {
    // For now, return a static error message
    // In a real implementation, you might want to use thread-local storage
    b"No error information available\0".as_ptr() as *const c_char
}

/// Get the last error message from a specific script
#[unsafe(no_mangle)]
pub extern "C" fn col_get_script_error(script: *mut COLScript) -> *const c_char {
    if script.is_null() {
        return ptr::null();
    }

    let script = unsafe { &*script };
    match &script.last_error {
        Some(error) => {
            // Convert to C string - note: this is not thread-safe and leaks memory
            // In production, you'd want a better approach
            match CString::new(error.as_str()) {
                Ok(c_str) => c_str.into_raw(),
                Err(_) => ptr::null(),
            }
        }
        None => ptr::null(),
    }
}

/// Initialize the COL runtime
#[unsafe(no_mangle)]
pub extern "C" fn col_initialize() -> COLResult {
    // Initialize LLVM if needed
    COLResult::Success
}

/// Shutdown the COL runtime
#[unsafe(no_mangle)]
pub extern "C" fn col_shutdown() {
    // Cleanup LLVM context, etc.
}

/// Function pointer type for print callback
pub type PrintCallback = extern "C" fn(*const c_char);

/// Static storage for print callback
static mut PRINT_CALLBACK: Option<PrintCallback> = None;

/// Register a print callback function
#[unsafe(no_mangle)]
pub extern "C" fn col_register_print_callback(callback: PrintCallback) {
    unsafe {
        PRINT_CALLBACK = Some(callback);
    }
}

/// Call the registered print callback with a message
#[unsafe(no_mangle)]
pub extern "C" fn col_print(message: *const c_char) -> COLResult {
    if message.is_null() {
        return COLResult::ErrorInvalidParameter;
    }

    unsafe {
        if let Some(callback) = PRINT_CALLBACK {
            callback(message);
            COLResult::Success
        } else {
            COLResult::ErrorInvalidHandle
        }
    }
}

/// Print a number value
#[unsafe(no_mangle)]
pub extern "C" fn col_print_number(value: c_double) -> COLResult {
    let message = format!("{}", value);
    match CString::new(message) {
        Ok(c_str) => col_print(c_str.as_ptr()),
        Err(_) => COLResult::ErrorInvalidParameter,
    }
}

/// Print a boolean value  
#[unsafe(no_mangle)]
pub extern "C" fn col_print_boolean(value: c_int) -> COLResult {
    let message = if value != 0 { "true" } else { "false" };
    match CString::new(message) {
        Ok(c_str) => col_print(c_str.as_ptr()),
        Err(_) => COLResult::ErrorInvalidParameter,
    }
}
