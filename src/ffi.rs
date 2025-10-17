// FFI (Foreign Function Interface) module for C# integration
// This module provides C-compatible interfaces for the COL runtime

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int};
use std::ptr;

use crate::codegen::ir_generator::IRGenerator;
use crate::handler::parse_handler::ParseHandler;
use crate::parser::program::Program;

/// Opaque handle to a compiled COL script
pub struct COLScript {
    program: Program,
    ir_generator: Option<IRGenerator<'static>>,
}

/// Result codes for C# interop
#[repr(C)]
pub enum COLResult {
    Success = 0,
    ErrorCompilation = 1,
    ErrorExecution = 2,
    ErrorInvalidHandle = 3,
    ErrorInvalidParameter = 4,
}

/// C-compatible value type for cross-language communication
#[repr(C)]
pub union COLValue {
    pub number: c_double,
    pub boolean: c_int,
    pub string_ptr: *mut c_char,
}

/// Tagged union for runtime values
#[repr(C)]
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
        ir_generator: None,
    }))
}

/// Call a function in the compiled script
#[unsafe(no_mangle)]
pub extern "C" fn col_call_function(
    script: *mut COLScript,
    function_name: *const c_char,
    _args: *const COLVariant,
    _arg_count: c_int,
    result: *mut COLVariant,
) -> COLResult {
    if script.is_null() || function_name.is_null() {
        return COLResult::ErrorInvalidHandle;
    }

    let _script = unsafe { &mut *script };
    let _func_name = match unsafe { CStr::from_ptr(function_name) }.to_str() {
        Ok(s) => s,
        Err(_) => return COLResult::ErrorInvalidParameter,
    };

    // TODO: Implement actual function execution
    // For now, return a dummy success
    if !result.is_null() {
        unsafe {
            (*result).value_type = 0; // number type
            (*result).value.number = 42.0;
        }
    }

    COLResult::Success
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

    // TODO: Implement global variable setting
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

    // TODO: Implement global variable getting
    unsafe {
        (*result).value_type = 3; // null type
    }
    COLResult::Success
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
    // TODO: Implement error message tracking
    ptr::null()
}

/// Initialize the COL runtime
#[unsafe(no_mangle)]
pub extern "C" fn col_initialize() -> COLResult {
    // TODO: Initialize LLVM context, etc.
    COLResult::Success
}

/// Shutdown the COL runtime
#[unsafe(no_mangle)]
pub extern "C" fn col_shutdown() {
    // TODO: Cleanup LLVM context, etc.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_compile_simple_script() {
        let source = CString::new("var x = 5;").unwrap();
        let script = col_compile_script(source.as_ptr());
        assert!(!script.is_null());
        col_destroy_script(script);
    }

    #[test]
    fn test_null_source() {
        let script = col_compile_script(ptr::null());
        assert!(script.is_null());
    }
}
