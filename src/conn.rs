use env::Environment;
use ffi::{
    OCISvcCtx, OCIServer, OCISession, OCIHandleType, OCIMode,
    OCIAttribute, OracleError, oci_handle_alloc, oci_server_attach, oci_attr_set
};
use libc::c_void;

pub struct Connection {
    environment:    Environment,
    service_handle: *mut OCISvcCtx,
    server_handle:  *mut OCIServer,
    // session_handle: *mut OCISession,
}

impl Connection {
    pub fn new() -> Result<Connection, OracleError> {
        let env = try!(Environment::new());

        let server_handle = try!(
            oci_handle_alloc(env.handle, OCIHandleType::Server)
        ) as *mut OCIServer;

        let service_handle = try!(
            oci_handle_alloc(env.handle, OCIHandleType::Service)
        ) as *mut OCISvcCtx;

        try!(
            oci_server_attach(server_handle, env.error_handle,
                              "bzzz".to_string(), OCIMode::Default)
        );

        // set attribute server context in the service context
        try!(
            oci_attr_set(service_handle as *mut c_void, OCIHandleType::Service,
                         server_handle as *mut c_void, OCIAttribute::Server, env.error_handle)
        );

        Ok(
            Connection {
                environment:    env,
                service_handle: service_handle,
                server_handle:  server_handle,
                // session_handle: sessionHandle,
            }
        )
    }
}
