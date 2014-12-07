use ffi;
use libc::c_void;

pub struct Environment {
    pub handle:       *mut ffi::OCIEnv,
    pub error_handle: *mut ffi::OCIError,
}

impl Environment {
    pub fn new() -> Result<Environment, ffi::OracleError> {
        let handle = try!(ffi::oci_env_nls_create(ffi::OCIMode::Default));
        let error_handle = try!(
            ffi::oci_handle_alloc(handle, ffi::OCIHandleType::Error)
        ) as *mut ffi::OCIError;
        Ok(Environment {handle: handle, error_handle: error_handle})
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        ffi::oci_handle_free(self.error_handle as *mut c_void, ffi::OCIHandleType::Error)
            .ok().expect("oci_handle_free (error_handle) failed");
        ffi::oci_handle_free(self.handle as *mut c_void, ffi::OCIHandleType::Environment)
            .ok().expect("oci_handle_free (environment handle) failed");
    }
}
