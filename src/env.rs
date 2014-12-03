use ffi::{OCIEnv, OCIError, OCIMode, OCIHandleType, oci_env_nls_create, oci_handle_alloc};

pub struct Environment {
    pub handle:       *mut OCIEnv,
    pub error_handle: *mut OCIError,
}

impl Environment {
    pub fn new() -> Environment {
        let handle = oci_env_nls_create(OCIMode::Default)
            .ok().expect("OCIEnvNlsCreate failed");
        let error_handle = oci_handle_alloc(handle, OCIHandleType::Error)
            .ok().expect("Cannot allocate Error handle") as *mut OCIError;
        Environment {handle: handle, error_handle: error_handle}
    }
}
