use ffi::{
    OCIEnv, OCIError, OCIMode, OCIHandleType, OracleError,
    oci_env_nls_create, oci_handle_alloc, oci_handle_free
};
use libc::c_void;

pub struct Environment {
    pub handle:       *mut OCIEnv,
    pub error_handle: *mut OCIError,
}

impl Environment {
    pub fn new() -> Result<Environment, OracleError> {
        let handle = try!(oci_env_nls_create(OCIMode::Default));
        let error_handle = try!(oci_handle_alloc(handle, OCIHandleType::Error)) as *mut OCIError;
        Ok(Environment {handle: handle, error_handle: error_handle})
    }
}

impl Drop for Environment {
    fn drop(&mut self) {
        oci_handle_free(self.error_handle as *mut c_void, OCIHandleType::Error)
            .ok().expect("oci_handle_free (error_handle) failed");
        oci_handle_free(self.handle as *mut c_void, OCIHandleType::Environment)
            .ok().expect("oci_handle_free (environment handle) failed");
    }
}
