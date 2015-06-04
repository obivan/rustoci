use rustoci_ffi;
use conn;

pub struct Statement {
    conn:        conn::Connection,
    stmt_handle: *mut rustoci_ffi::OCIStmt,
    stmt_hash:   String,
}

impl Statement {
    pub fn new(conn: conn::Connection, stmt_text: String) -> Result<Statement, rustoci_ffi::OracleError> {
        let stmt_hash = stmt_text.clone(); // hashing is currently unstable
        let stmt_handle = try!(rustoci_ffi::oci_stmt_prepare2(conn.service_handle,
                                                              conn.env.error_handle,
                                                              &stmt_text, &stmt_hash));
        Ok(Statement {conn: conn, stmt_handle: stmt_handle, stmt_hash: stmt_hash})
    }

    pub fn execute(self) -> Result<Statement, rustoci_ffi::OracleError> {
        try!(rustoci_ffi::oci_stmt_execute(self.conn.service_handle,
                                           self.stmt_handle,
                                           self.conn.env.error_handle));
        Ok(self)
    }
}

impl Drop for Statement {
    fn drop(&mut self) {
        rustoci_ffi::oci_stmt_release(self.stmt_handle, self.conn.env.error_handle, &self.stmt_hash)
            .ok().expect("oci_stmt_release failed");
    }
}
