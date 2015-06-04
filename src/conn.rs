use env;
use rustoci_ffi;
use stmt;

pub struct Connection {
    pub env:            env::Environment,
    pub service_handle: *mut rustoci_ffi::OCISvcCtx,
    server_handle:      *mut rustoci_ffi::OCIServer,
    session_handle:     *mut rustoci_ffi::OCISession,
}

impl Connection {
    pub fn new(username: String,
               password: String,
               database: String) -> Result<Connection, rustoci_ffi::OracleError> {
        // Initialize environment
        let env = try!(env::Environment::new());

        // Allocate the server handle
        let server_handle =
            try!(rustoci_ffi::oci_handle_alloc(env.handle,
                 rustoci_ffi::OCIHandleType::Server)) as *mut rustoci_ffi::OCIServer;

        // Allocate the service context handle
        let service_handle =
            try!(rustoci_ffi::oci_handle_alloc(env.handle,
                 rustoci_ffi::OCIHandleType::Service)) as *mut rustoci_ffi::OCISvcCtx;

        // Allocate the session handle
        let session_handle =
            try!(rustoci_ffi::oci_handle_alloc(env.handle,
                 rustoci_ffi::OCIHandleType::Session)) as *mut rustoci_ffi::OCISession;

        try!(rustoci_ffi::oci_server_attach(server_handle, env.error_handle,
                                            database, rustoci_ffi::OCIMode::Default));

        // Set attribute server context in the service context
        try!(rustoci_ffi::oci_attr_set(service_handle as *mut rustoci_ffi::c_void,
                                       rustoci_ffi::OCIHandleType::Service,
                                       server_handle as *mut rustoci_ffi::c_void,
                                       rustoci_ffi::OCIAttribute::Server,
                                       env.error_handle));

        // Set attribute username in the session context
        try!(rustoci_ffi::oci_attr_set(session_handle as *mut rustoci_ffi::c_void,
                                       rustoci_ffi::OCIHandleType::Session,
                                       username.as_ptr() as *mut rustoci_ffi::c_void,
                                       rustoci_ffi::OCIAttribute::Username, env.error_handle));

        // Set attribute password in the session context
        try!(rustoci_ffi::oci_attr_set(session_handle as *mut rustoci_ffi::c_void,
                                       rustoci_ffi::OCIHandleType::Session,
                                       password.as_ptr() as *mut rustoci_ffi::c_void,
                                       rustoci_ffi::OCIAttribute::Password, env.error_handle));

        // Begin session
        try!(rustoci_ffi::oci_session_begin(service_handle, env.error_handle, session_handle,
                                            rustoci_ffi::OCICredentialsType::Rdbms,
                                            rustoci_ffi::OCIAuthMode::Default));

        // Set session context in the service context
        try!(rustoci_ffi::oci_attr_set(service_handle as *mut rustoci_ffi::c_void,
                                       rustoci_ffi::OCIHandleType::Service,
                                       session_handle as *mut rustoci_ffi::c_void,
                                       rustoci_ffi::OCIAttribute::Session,
                                       env.error_handle));

        Ok(
            Connection {
                env:            env,
                service_handle: service_handle,
                server_handle:  server_handle,
                session_handle: session_handle,
            }
        )
    }

    pub fn new_statement(self, stmt_text: String) -> Result<stmt::Statement, rustoci_ffi::OracleError> {
        stmt::Statement::new(self, stmt_text)
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        rustoci_ffi::oci_session_end(self.service_handle, self.env.error_handle, self.session_handle)
            .ok().expect("oci_session_end failed");
        rustoci_ffi::oci_server_detach(self.server_handle, self.env.error_handle)
            .ok().expect("oci_server_detach failed");
        rustoci_ffi::oci_handle_free(self.session_handle as *mut rustoci_ffi::c_void,
                                     rustoci_ffi::OCIHandleType::Session)
            .ok().expect("oci_handle_free (session_handle) failed");
        rustoci_ffi::oci_handle_free(self.service_handle as *mut rustoci_ffi::c_void,
                                     rustoci_ffi::OCIHandleType::Service)
            .ok().expect("oci_handle_free (service_handle) failed");
        rustoci_ffi::oci_handle_free(self.server_handle as *mut rustoci_ffi::c_void,
                                     rustoci_ffi::OCIHandleType::Server)
            .ok().expect("oci_handle_free (server_handle) failed");
    }
}
