use env::Environment;
use ffi::{
    OCISvcCtx, OCIServer, OCISession, OCIHandleType, OCIMode,
    OCIAttribute, oci_handle_alloc, oci_server_attach, oci_attr_set
};
use libc::c_void;

pub struct Connection {
    environment:    Environment,
    service_handle: *mut OCISvcCtx,
    server_handle:  *mut OCIServer,
    // session_handle: *mut OCISession,
}

impl Connection {
    pub fn new() -> Connection {
        let env = Environment::new();

        let server_handle = oci_handle_alloc(env.handle, OCIHandleType::Server)
            .ok().expect("Cannot allocate Server handle") as *mut OCIServer;

        let service_handle = oci_handle_alloc(env.handle, OCIHandleType::Service)
            .ok().expect("Cannot allocate Service handle") as *mut OCISvcCtx;

        match oci_server_attach(server_handle, env.error_handle,
                                "bzzz".to_string(), OCIMode::Default) {
            Ok(_) => (),
            Err(e) => panic!("oci_server_attach failed with error: {}", e),
        };

        // set attribute server context in the service context
        match oci_attr_set(service_handle as *mut c_void, OCIHandleType::Service,
                           server_handle as *mut c_void, OCIAttribute::Server, env.error_handle) {
            Ok(_) => (),
            Err(e) => panic!("oci_attr_set failed with error: {}", e),
        };

        Connection {
            environment:    env,
            service_handle: service_handle,
            server_handle:  server_handle,
            // session_handle: sessionHandle,
        }
    }
}
