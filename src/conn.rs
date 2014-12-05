use env::Environment;
use ffi::{
    OCISvcCtx, OCIServer, OCISession, OCIHandleType, OCICredentialsType, OCIMode, OCIAuthMode,
    OCIAttribute, OracleError, oci_handle_alloc, oci_server_attach, oci_attr_set, oci_session_begin
};
use libc::c_void;

pub struct Connection {
    environment:    Environment,
    service_handle: *mut OCISvcCtx,
    server_handle:  *mut OCIServer,
    session_handle: *mut OCISession,
}

impl Connection {
    pub fn new() -> Result<Connection, OracleError> {

        // Initialize environment
        let env = try!(Environment::new());

        // Allocate the server handle
        let server_handle = try!(
            oci_handle_alloc(env.handle, OCIHandleType::Server)
        ) as *mut OCIServer;

        // Allocate the service context handle
        let service_handle = try!(
            oci_handle_alloc(env.handle, OCIHandleType::Service)
        ) as *mut OCISvcCtx;

        // Allocate the session handle
        let session_handle = try!(
            oci_handle_alloc(env.handle, OCIHandleType::Session)
        ) as *mut OCISession;

        try!(
            oci_server_attach(server_handle, env.error_handle,
                              "bzzz".to_string(), OCIMode::Default)
        );

        // Set attribute server context in the service context
        try!(
            oci_attr_set(service_handle as *mut c_void, OCIHandleType::Service,
                         server_handle as *mut c_void, OCIAttribute::Server, env.error_handle)
        );

        // Set attribute username in the session context
        try!(
            "apps".with_c_str(|username|
                oci_attr_set(session_handle as *mut c_void, OCIHandleType::Session,
                             username as *mut c_void, OCIAttribute::Username, env.error_handle)
            )
        );

        // Set attribute password in the session context
        try!(
            "apps".with_c_str(|password|
                oci_attr_set(session_handle as *mut c_void, OCIHandleType::Session,
                             password as *mut c_void, OCIAttribute::Password, env.error_handle)
            )
        );

        // Begin session
        try!(
            oci_session_begin(service_handle, env.error_handle, session_handle,
                              OCICredentialsType::Rdbms, OCIAuthMode::Default)
        );

        // Set session context in the service context
        try!(
            oci_attr_set(service_handle as *mut c_void, OCIHandleType::Service,
                         session_handle as *mut c_void, OCIAttribute::Session, env.error_handle)
        );

        Ok(
            Connection {
                environment:    env,
                service_handle: service_handle,
                server_handle:  server_handle,
                session_handle: session_handle,
            }
        )
    }
}
