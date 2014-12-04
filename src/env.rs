use ffi::{
    OCIEnv, OCIError, OCIMode, OCIHandleType, OracleError, oci_env_nls_create, oci_handle_alloc
};

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
