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
}
