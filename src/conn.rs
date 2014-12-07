use env;
use ffi;
use stmt;
use libc::c_void;

pub struct Connection {
    pub env:        env::Environment,
    service_handle: *mut ffi::OCISvcCtx,
    server_handle:  *mut ffi::OCIServer,
    session_handle: *mut ffi::OCISession,
}

impl Connection {
    pub fn new(username: String,
               password: String,
               database: String) -> Result<Connection, ffi::OracleError> {
        // Initialize environment
        let env = try!(env::Environment::new());

        // Allocate the server handle
        let server_handle = try!(
            ffi::oci_handle_alloc(env.handle, ffi::OCIHandleType::Server)
        ) as *mut ffi::OCIServer;

        // Allocate the service context handle
        let service_handle = try!(
            ffi::oci_handle_alloc(env.handle, ffi::OCIHandleType::Service)
        ) as *mut ffi::OCISvcCtx;

        // Allocate the session handle
        let session_handle = try!(
            ffi::oci_handle_alloc(env.handle, ffi::OCIHandleType::Session)
        ) as *mut ffi::OCISession;

        try!(
            ffi::oci_server_attach(server_handle, env.error_handle,
                                   database, ffi::OCIMode::Default)
        );

        // Set attribute server context in the service context
        try!(
            ffi::oci_attr_set(service_handle as *mut c_void, ffi::OCIHandleType::Service,
                              server_handle as *mut c_void, ffi::OCIAttribute::Server,
                              env.error_handle)
        );

        // Set attribute username in the session context
        try!(
            username.with_c_str(|u|
                ffi::oci_attr_set(session_handle as *mut c_void, ffi::OCIHandleType::Session,
                                  u as *mut c_void, ffi::OCIAttribute::Username, env.error_handle)
            )
        );

        // Set attribute password in the session context
        try!(
            password.with_c_str(|p|
                ffi::oci_attr_set(session_handle as *mut c_void, ffi::OCIHandleType::Session,
                                  p as *mut c_void, ffi::OCIAttribute::Password, env.error_handle)
            )
        );

        // Begin session
        try!(
            ffi::oci_session_begin(service_handle, env.error_handle, session_handle,
                                   ffi::OCICredentialsType::Rdbms, ffi::OCIAuthMode::Default)
        );

        // Set session context in the service context
        try!(
            ffi::oci_attr_set(service_handle as *mut c_void, ffi::OCIHandleType::Service,
                              session_handle as *mut c_void, ffi::OCIAttribute::Session,
                              env.error_handle)
        );

        Ok(
            Connection {
                env:            env,
                service_handle: service_handle,
                server_handle:  server_handle,
                session_handle: session_handle,
            }
        )
    }

    pub fn new_statement(self) -> Result<stmt::Statement, ffi::OracleError> {
        stmt::Statement::new(self)
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        ffi::oci_session_end(self.service_handle, self.env.error_handle, self.session_handle)
            .ok().expect("oci_session_end failed");
        ffi::oci_server_detach(self.server_handle, self.env.error_handle)
            .ok().expect("oci_server_detach failed");
        ffi::oci_handle_free(self.session_handle as *mut c_void, ffi::OCIHandleType::Session)
            .ok().expect("oci_handle_free (session_handle) failed");
        ffi::oci_handle_free(self.service_handle as *mut c_void, ffi::OCIHandleType::Service)
            .ok().expect("oci_handle_free (service_handle) failed");
        ffi::oci_handle_free(self.server_handle as *mut c_void, ffi::OCIHandleType::Server)
            .ok().expect("oci_handle_free (server_handle) failed");
    }
}
