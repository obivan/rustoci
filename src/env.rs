use rustoci_ffi;

pub struct Environment {
    pub handle:       *mut rustoci_ffi::OCIEnv,
    pub error_handle: *mut rustoci_ffi::OCIError,
}

impl Environment {
    pub fn new() -> Result<Environment, rustoci_ffi::OracleError> {
        let handle = try!(rustoci_ffi::oci_env_nls_create(rustoci_ffi::OCIMode::Default));
        let error_handle = try!(
            rustoci_ffi::oci_handle_alloc(handle, rustoci_ffi::OCIHandleType::Error)
        ) as *mut rustoci_ffi::OCIError;
        Ok(Environment {handle: handle, error_handle: error_handle})
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        rustoci_ffi::oci_handle_free(self.error_handle as *mut rustoci_ffi::c_void,
                                     rustoci_ffi::OCIHandleType::Error)
            .ok().expect("oci_handle_free (error_handle) failed");
        rustoci_ffi::oci_handle_free(self.handle as *mut rustoci_ffi::c_void,
                                     rustoci_ffi::OCIHandleType::Environment)
            .ok().expect("oci_handle_free (environment handle) failed");
    }
}
