use ffi;
use conn;

pub struct Statement {
    conn:        conn::Connection,
    stmt_handle: *mut ffi::OCIStmt,
}

impl Statement {
    pub fn new(conn: conn::Connection) -> Result<Statement, ffi::OracleError> {
        let stmt_handle = try!(
            ffi::oci_handle_alloc(conn.env.handle, ffi::OCIHandleType::Statement)
        ) as *mut ffi::OCIStmt;
        Ok(Statement {conn: conn, stmt_handle: stmt_handle})
    }

    pub fn prepare(self, stmt_text: String) -> Result<Statement, ffi::OracleError> {
        let stmt_handle = try!(
            ffi::oci_stmt_prepare2(self.conn.service_handle, self.conn.env.error_handle,
                                   stmt_text, None)
        );
        Ok(Statement {conn: self.conn, stmt_handle: stmt_handle})
    }

}
