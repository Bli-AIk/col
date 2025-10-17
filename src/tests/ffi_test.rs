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
}
