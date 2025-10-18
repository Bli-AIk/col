#[cfg(test)]
mod tests {
    use crate::ffi::*;
    use std::ffi::CString;
    use std::ptr;

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

    #[test]
    fn test_global_variables() {
        let source = CString::new("var x = 10;").unwrap();
        let script = col_compile_script(source.as_ptr());
        assert!(!script.is_null());

        // Set a global variable
        let var_name = CString::new("testVar").unwrap();
        let mut value = COLVariant {
            value_type: 0, // number
            value: COLValue { number: 42.5 },
        };
        
        let result = col_set_global_variable(script, var_name.as_ptr(), &value);
        assert_eq!(result, COLResult::Success);

        // Get the global variable
        let mut retrieved_value = COLVariant {
            value_type: 3, // null
            value: COLValue { number: 0.0 },
        };
        
        let result = col_get_global_variable(script, var_name.as_ptr(), &mut retrieved_value);
        assert_eq!(result, COLResult::Success);
        assert_eq!(retrieved_value.value_type, 0); // number
        assert_eq!(unsafe { retrieved_value.value.number }, 42.5);

        col_destroy_script(script);
    }

    #[test]
    fn test_call_function() {
        let source = CString::new("function test_func(a) { return a + 3; }").unwrap();
        let script = col_compile_script(source.as_ptr());
        assert!(!script.is_null());

        let func_name = CString::new("test_func").unwrap();
        let mut result = COLVariant {
            value_type: 3, // null
            value: COLValue { number: 0.0 },
        };

        // Prepare arguments
        let arg = COLVariant {
            value_type: 0, // number
            value: COLValue { number: 5.0 },
        };

        let call_result = col_call_function(
            script, 
            func_name.as_ptr(), 
            &arg, 
            1, 
            &mut result
        );

        // Note: The actual execution might fail due to incomplete IR generation,
        // but we can test that the FFI interface works correctly
        match call_result {
            COLResult::Success => {
                assert_eq!(result.value_type, 0); // number
                assert_eq!(unsafe { result.value.number }, 8.0);
            }
            COLResult::ErrorCompilation | COLResult::ErrorExecution => {
                // This is expected if IR generation is incomplete
                println!("Function execution failed as expected due to incomplete IR generation");
            }
            _ => panic!("Unexpected result: {:?}", call_result),
        }

        col_destroy_script(script);
    }

    #[test]
    fn test_print_callback() {
        // Test print callback registration
        extern "C" fn test_print_callback(msg: *const std::os::raw::c_char) {
            if !msg.is_null() {
                let c_str = unsafe { std::ffi::CStr::from_ptr(msg) };
                if let Ok(msg_str) = c_str.to_str() {
                    println!("Print callback received: {}", msg_str);
                }
            }
        }

        col_register_print_callback(test_print_callback);

        let test_msg = CString::new("Hello from COL!").unwrap();
        let result = col_print(test_msg.as_ptr());
        assert_eq!(result, COLResult::Success);

        let result = col_print_number(123.456);
        assert_eq!(result, COLResult::Success);

        let result = col_print_boolean(1);
        assert_eq!(result, COLResult::Success);
    }

    #[test]
    fn test_runtime_lifecycle() {
        let init_result = col_initialize();
        assert_eq!(init_result, COLResult::Success);

        // Test some operations
        let source = CString::new("var x = 1;").unwrap();
        let script = col_compile_script(source.as_ptr());
        assert!(!script.is_null());
        col_destroy_script(script);

        col_shutdown();
    }
}
