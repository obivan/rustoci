use ffi;
use conn;
use std::hash::{hash, SipHasher};

pub struct Statement {
    conn:        conn::Connection,
    stmt_handle: *mut ffi::OCIStmt,
    stmt_hash:   String,
}

impl Statement {
    pub fn new(conn: conn::Connection, stmt_text: String) -> Result<Statement, ffi::OracleError> {
        let stmt_hash = hash::<_, SipHasher>(&stmt_text).to_string();
        let stmt_handle = try!(
            ffi::oci_stmt_prepare2(conn.service_handle, conn.env.error_handle,
                                   &stmt_text, &stmt_hash)
        );
        Ok(Statement {conn: conn, stmt_handle: stmt_handle, stmt_hash: stmt_hash})
    }

    pub fn execute(self) -> Result<Statement, ffi::OracleError> {
        try!(
            ffi::oci_stmt_execute(self.conn.service_handle,
                                  self.stmt_handle,
                                  self.conn.env.error_handle)
        );
        Ok(self)
    }
}

impl Drop for Statement {
    fn drop(&mut self) {
        ffi::oci_stmt_release(self.stmt_handle, self.conn.env.error_handle, &self.stmt_hash)
            .ok().expect("oci_stmt_release failed");
    }
}
