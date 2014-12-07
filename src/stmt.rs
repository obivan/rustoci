use ffi;
use conn;

pub struct Statement {
    conn:        conn::Connection,
    stmt_handle: *mut ffi::OCIStmt,
}

impl Statement {
    pub fn new(conn: conn::Connection, stmt_text: String) -> Result<Statement, ffi::OracleError> {
        let stmt_handle = try!(
            ffi::oci_stmt_prepare2(conn.service_handle, conn.env.error_handle, stmt_text, None)
        );
        Ok(Statement {conn: conn, stmt_handle: stmt_handle})
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
