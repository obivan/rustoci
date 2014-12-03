use env::Environment;
use ffi::{OCISvcCtx, OCIServer, OCISession, OCIHandleType, OCIMode, oci_handle_alloc, oci_server_attach};

pub struct Connection {
    environment:    Environment,
    handle:         *mut OCISvcCtx,
    server_handle:  *mut OCIServer,
    // session_handle: *mut OCISession,
}

impl Connection {
    pub fn new() -> Connection {
        let env = Environment::new();
        let server_handle = oci_handle_alloc(env.handle, OCIHandleType::Server)
            .ok().expect("Cannot allocate Server handle") as *mut OCIServer;
        let handle = oci_handle_alloc(env.handle, OCIHandleType::Service)
            .ok().expect("Cannot allocate Service handle") as *mut OCISvcCtx;
        let attach_result = oci_server_attach(server_handle, env.error_handle, "bzzz".to_string(), OCIMode::Default);
        match attach_result {
            Ok(_) => (),
            Err(e) => panic!("oci_server_attach failed with error: {}", e),
        }
        Connection {
            environment:    env,
            handle:         handle,
            server_handle:  server_handle,
            // session_handle: sessionHandle,
        }
    }
}