use libc::{c_void, c_ushort, c_ulong, c_uchar, c_char, c_uint, c_int};
use std::c_str::CString;
use std::error;
use std::fmt;
use std::ptr;

#[repr(C)]
pub struct OCIEnv;

#[repr(C)]
pub struct OCIError;

#[repr(C)]
pub struct OCISvcCtx;

#[repr(C)]
pub struct OCIServer;

#[repr(C)]
pub struct OCISession;

#[repr(C)]
pub struct OCIStmt;

#[allow(dead_code)]
pub enum OCIMode {
    // OCI_DEFAULT - The default value, which is non-UTF-16 encoding.
    Default = 0x00000000,

    // OCI_THREADED - Uses threaded environment.
    // Internal data structures not exposed to the user are protected from concurrent
    // accesses by multiple threads.
    Threaded = 0x00000001,

    // OCI_OBJECT - Uses object features.
    Object = 0x00000002,

    // OCI_EVENTS - Uses publish-subscribe notifications.
    Events = 0x00000004,

    // OCI_NO_UCB - Suppresses the calling of the dynamic callback routine OCIEnvCallback().
    // The default behavior is to allow calling of OCIEnvCallback() when the environment is created.
    NoUcb = 0x00000040,

    // OCI_NO_MUTEX - No mutual exclusion (mutex) locking occurs in this mode.
    // All OCI calls done on the environment handle, or on handles derived from the environment
    // handle, must be serialized.
    // OCI_THREADED must also be specified when OCI_NO_MUTEX is specified.
    NoMutex = 0x00000080,

    // OCI_SUPPRESS_NLS_VALIDATION - Suppresses NLS character validation;
    // NLS character validation suppression is on by default beginning with
    // Oracle Database 11g Release 1 (11.1). Use OCI_ENABLE_NLS_VALIDATION to
    // enable NLS character validation.
    SuppressNLSValidation = 0x00100000,

    // OCI_NCHAR_LITERAL_REPLACE_ON - Turns on N' substitution.
    NcharLiteralReplaceOn = 0x00400000,

    // OCI_NCHAR_LITERAL_REPLACE_OFF - Turns off N' substitution.
    // If neither this mode nor OCI_NCHAR_LITERAL_REPLACE_ON is used, the substitution is
    // determined by the environment variable ORA_NCHAR_LITERAL_REPLACE, which can be set
    // to TRUE or FALSE. When it is set to TRUE, the replacement is turned on; otherwise
    // it is turned off, which is the default setting in OCI.
    NcharLiteralReplaceOff = 0x00800000,

    // OCI_ENABLE_NLS_VALIDATION - Enables NLS character validation.
    EnableNLSValidation = 0x01000000,
}

pub struct OracleError {
    code:     int,
    message:  String,
    location: String,
}

impl fmt::Show for OracleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!{f, "\n\n  Error code: {}\n  Error message: {}\n  Where: {}\n\n",
               self.code, self.message, self.location}
    }
}

impl error::Error for OracleError {
    fn description(&self) -> &str {
        "Oracle error"
    }

    fn detail(&self) -> Option<String> {
        Some(format!("{}", self))
    }
}

#[allow(dead_code)]
pub enum OCIHandleType {
    Environment = 1,  // OCI_HTYPE_ENV
    Error       = 2,  // OCI_HTYPE_ERROR
    Service     = 3,  // OCI_HTYPE_SVCCTX
    Statement   = 4,  // OCI_HTYPE_STMT
    Bind        = 5,  // OCI_HTYPE_BIND
    Define      = 6,  // OCI_HTYPE_DEFINE
    Describe    = 7,  // OCI_HTYPE_DESCRIBE
    Server      = 8,  // OCI_HTYPE_SERVER
    Session     = 9,  // OCI_HTYPE_SESSION
    Transaction = 10, // OCI_HTYPE_TRANS
}

#[allow(dead_code)]
pub enum OCICredentialsType {
    Rdbms    = 1, // OCI_CRED_RDBMS
    External = 2, // OCI_CRED_EXT
}

#[allow(dead_code)]
pub enum OCIAuthMode {
    Default    = 0x00000000, // OCI_DEFAULT
    Migrate    = 0x00000001, // OCI_MIGRATE
    Sysdba     = 0x00000002, // OCI_SYSDBA
    Sysoper    = 0x00000004, // OCI_SYSOPER
    PrelimAuth = 0x00000008, // OCI_PRELIM_AUTH
    StmtCache  = 0x00000040, // OCI_STMT_CACHE
}

pub enum OCIAttribute {
    // OCI_ATTR_SERVER
    // Mode: READ/WRITE
    // When read, returns the pointer to the server context attribute of the service context.
    // When changed, sets the server context attribute of the service context.
    // Attribute Data Type: OCIServer ** / OCIServer *
    Server = 6,

    // OCI_ATTR_SESSION
    // Mode: READ/WRITE
    // When read, returns the pointer to the authentication context attribute of
    // the service context.
    // When changed, sets the authentication context attribute of the service context.
    // Attribute Data Type: OCISession **/ OCISession *
    Session = 7,

    // OCI_ATTR_USERNAME
    // Mode: READ/WRITE
    // Specifies a user name to use for authentication.
    // Attribute Data Type: oratext **/oratext * [oratext = c_uchar]
    Username = 22,

    // OCI_ATTR_PASSWORD
    // Mode: WRITE
    // Specifies a password to use for authentication.
    // Attribute Data Type: oratext * [oratext = c_uchar]
    Password = 23,
}

#[link(name = "clntsh")]
extern "C" {
    // Creates and initializes an environment handle for OCI functions to work under.
    // It is an enhanced version of the OCIEnvCreate() function.
    // This call should be invoked before any other OCI call and should be used instead
    // of the OCIInitialize() call.
    // 
    // This call returns an environment handle, which is then used by the remaining OCI functions.
    // There can be multiple environments in OCI, each with its own environment modes.
    // This function also performs any process level initialization if required by any mode.
    // For example, if the user wants to initialize an environment as OCI_THREADED, then all
    // libraries that are used by OCI are also initialized in the threaded mode.
    // 
    // After you use OCIEnvNlsCreate() to create the environment handle, the actual lengths and
    // returned lengths of bind and define handles are always expressed in number of bytes.
    // This applies to the following calls:
    //   OCIBindByName()
    //   OCIBindByPos()
    //   OCIBindDynamic()
    //   OCIDefineByPos()
    //   OCIDefineDynamic()
    // 
    // This function sets nonzero charset and ncharset as client-side database and national
    // character sets, replacing the ones specified by NLS_LANG and NLS_NCHAR.
    // When charset and ncharset are 0, the function behaves exactly the same as OCIEnvCreate().
    // Specifically, charset controls the encoding for metadata and data with implicit form
    // attribute, and ncharset controls the encoding for data with SQLCS_NCHAR form attribute.
    // 
    // Although OCI_UTF16ID can be set by OCIEnvNlsCreate(), it cannot be set in NLS_LANG or
    // NLS_NCHAR. To access the character set IDs in NLS_LANG and NLS_NCHAR, use
    // OCINlsEnvironmentVariableGet().
    // 
    // If N' substitution is turned on, the OCIStmtPrepare() or OCIStmtPrepare2() function performs
    // the N' substitution on the SQL text and stores the resulting SQL text in the statement
    // handle. Thus, if the application uses OCI_ATTR_STATEMENT to retrieve the SQL text from the
    // OCI statement handle, the modified SQL text, instead of the original SQL text, is returned.
    // To turn on N' substitution in ksh shell: export ORA_NCHAR_LITERAL_REPLACE=TRUE
    // To turn on N' substitution in csh shell: setenv ORA_NCHAR_LITERAL_REPLACE TRUE
    // If a remote database is of a release before 10.2, N' substitution is not performed.
    // 
    // Regarding OCI_SUPPRESS_NLS_VALIDATION and OCI_ENABLE_NLS_VALIDATION modes, by default, when
    // client and server character sets are identical, and client and server releases are both
    // Oracle Database 11g Release 1 (11.1) or higher, OCI does not validate character data in the
    // interest of better performance. This means that if the application inserts a character
    // string with partial multibyte characters (for example, at the end of a bind variable), then
    // such strings could get persisted in the database as is.
    // 
    // Note that if either the client or the server release is older than
    // Oracle Database 11g Release 1 (11.1), then OCI does not allow partial characters.
    // 
    // The OCI_ENABLE_NLS_VALIDATION mode, which was the default until Oracle Database 10g
    // Release 2 (10.2), ensures that partial multibyte characters are not persisted in the
    // database (when client and server character sets are identical). If the application can
    // produce partial multibyte characters, and if the application can run in an environment where
    // the client and server character sets are identical, then Oracle recommends using the
    // OCI_ENABLE_NLS_VALIDATION mode explicitly in order to ensure that such partial
    // characters get stripped out.
    fn OCIEnvNlsCreate(
        // envp (OUT)
        // A pointer to an environment handle whose encoding setting is specified by mode.
        // The setting is inherited by statement handles derived from envp.
        envp: *mut *mut OCIEnv,

        // mode (IN)
        // Specifies initialization of the mode. See valid modes in OCIMode enum.
        mode: c_uint,

        // ctxp (IN)
        // Specifies the user-defined context for the memory callback routines.
        ctxp: *mut c_void,

        // malocfp (IN)
        // Specifies the user-defined memory allocation function.
        // If mode is OCI_THREADED, this memory allocation routine must be thread-safe.
        malocfp: Option<extern "C" fn (
            // ctxp (IN)
            // Specifies the context pointer for the user-defined memory allocation function.
            ctxp: *mut c_void,

            // size (IN)
            // Specifies the size of memory to be allocated by the user-defined
            // memory allocation function.
            size: c_ulong
        ) -> *mut c_void>,

        // ralocfp (IN)
        // Specifies the user-defined memory reallocation function.
        // If the mode is OCI_THREADED, this memory allocation routine must be thread-safe.
        ralocfp: Option<extern "C" fn (
            // ctxp (IN)
            // Specifies the context pointer for the user-defined memory reallocation function.
            ctxp: c_void,

            // memptr (IN)
            // Pointer to memory block.
            memptr: c_void,

            // newsize (IN)
            // Specifies the new size of memory to be allocated.
            newsize: c_ulong
        ) -> *mut c_void>,

        // mfreefp (IN)
        // Specifies the user-defined memory free function.
        // If the mode is OCI_THREADED, this memory free routine must be thread-safe.
        mfreefp: Option<extern "C" fn (
            // ctxp (IN)
            // Specifies the context pointer for the user-defined memory free function.
            ctxp: *mut c_void,

            // memptr (IN)
            // Pointer to memory to be freed.
            memptr: *mut c_void
        )>,

        // xtramemsz (IN)
        // Specifies the amount of user memory to be allocated for the duration of the environment.
        xtramem_sz: c_ulong,

        // usrmempp (OUT)
        // Returns a pointer to the user memory of size xtramemsz
        // allocated by the call for the user.
        usrmempp: *mut *mut c_void,

        // charset (IN)
        // The client-side character set for the current environment handle.
        // If it is 0, the NLS_LANG setting is used. OCI_UTF16ID is a valid setting;
        // it is used by the metadata and the CHAR data.
        charset: c_ushort,

        // ncharset (IN)
        // The client-side national character set for the current environment handle.
        // If it is 0, NLS_NCHAR setting is used. OCI_UTF16ID is a valid setting;
        // it is used by the NCHAR data.
        ncharset: c_ushort
    ) -> c_int;

    // Returns a pointer to an allocated and initialized handle, corresponding
    // to the type specified in type. A non-NULL handle is returned on success.
    // All handles are allocated with respect to an environment handle
    // that is passed in as a parent handle.
    // 
    // No diagnostics are available on error.
    // This call returns OCI_SUCCESS if successful,
    // or OCI_INVALID_HANDLE if an error occurs.
    // 
    // Handles must be allocated using OCIHandleAlloc() before they
    // can be passed into an OCI call.
    fn OCIHandleAlloc(
        // parenth (IN)
        // An environment handle.
        parenth: *const c_void,

        // hndlpp (OUT)
        // Returns a handle.
        hndlpp: *mut *mut c_void,

        // type (IN)
        // Specifies the type of handle to be allocated.
        _type: c_uint,

        // xtramem_sz (IN)
        // Specifies an amount of user memory to be allocated.
        xtramem_sz: c_ulong,

        // usrmempp (OUT)
        // Returns a pointer to the user memory of size xtramem_sz allocated
        // by the call for the user.
        usrmempp: *mut *mut c_void
    ) -> c_int;

    // Creates an access path to a data source for OCI operations.
    // 
    // This call is used to create an association between an OCI
    // application and a particular server.
    // 
    // This call assumes that OCIConnectionPoolCreate() has been called, giving poolName,
    // when connection pooling is in effect.
    // 
    // This call initializes a server context handle, which must have been previously allocated
    // with a call to OCIHandleAlloc(). The server context handle initialized by this call can
    // be associated with a service context through a call to OCIAttrSet(). After that association
    // has been made, OCI operations can be performed against the server.
    // 
    // If an application is operating against multiple servers, multiple server context handles
    // can be maintained. OCI operations are performed against whichever server context is
    // currently associated with the service context.
    // 
    // When OCIServerAttach() is successfully completed, an Oracle Database shadow process
    // is started. OCISessionEnd() and OCIServerDetach() should be called to clean up the
    // Oracle Database shadow process. Otherwise, the shadow processes accumulate and cause the
    // Linux or UNIX system to run out of processes. If the database is restarted and there are
    // not enough processes, the database may not start up.
    fn OCIServerAttach(
        // srvhp (IN/OUT)
        // An uninitialized server handle, which is initialized by this call.
        // Passing in an initialized server handle causes an error.
        srvhp: *mut OCIServer,

        // errhp (IN/OUT)
        // An error handle that you can pass to OCIErrorGet() for diagnostic
        // information when there is an error.
        errhp: *mut OCIError,

        // dblink (IN)
        // Specifies the database server to use. This parameter points to a character
        // string that specifies a connect string or a service point. If the connect
        // string is NULL, then this call attaches to the default host.
        // The string itself could be in UTF-16 encoding mode or not, depending on the
        // mode or the setting in application's environment handle. The length of dblink
        // is specified in dblink_len. The dblink pointer may be freed by the caller on return.
        // 
        // The name of the connection pool to connect to when mode = OCI_CPOOL.
        // This must be the same as the poolName parameter of the connection pool
        // created by OCIConnectionPoolCreate(). Must be in the encoding specified by the charset
        // parameter of a previous call to OCIEnvNlsCreate().
        dblink: *const c_uchar,

        // dblink_len (IN)
        // The length of the string pointed to by dblink. For a valid connect string name or
        // alias, dblink_len must be nonzero. Its value is in number of bytes.
        // 
        // The length of poolName, in number of bytes, regardless of
        // the encoding, when mode = OCI_CPOOL.
        dblink_len: c_int,

        // mode (IN)
        // Specifies the various modes of operation. The valid modes are:
        //   OCI_DEFAULT - For encoding, this value tells the server
        //     handle to use the setting in the environment handle.
        //   OCI_CPOOL - Use connection pooling.
        // Because an attached server handle can be set for any connection session handle,
        // the mode value here does not contribute to any session handle.
        mode: c_uint
    ) -> c_int;

    // Returns an error message in the buffer provided and an Oracle Database error code.
    // This function does not support SQL statements. Usually, hndlp is actually the error handle,
    // or the environment handle. You should always get the message in the encoding that
    // was set in the environment handle. This function can be called multiple times if there are
    // multiple diagnostic records for an error.
    // 
    // Note that OCIErrorGet() must not be called when the return code is OCI_SUCCESS.
    // Otherwise, an error message from a previously executed statement is found by OCIErrorGet().
    // 
    // The error handle is originally allocated with a call to OCIHandleAlloc().
    // 
    // Multiple diagnostic records can be retrieved by calling OCIErrorGet() repeatedly until
    // there are no more records (OCI_NO_DATA is returned). OCIErrorGet() returns
    // at most a single diagnostic record.
    fn OCIErrorGet(
        // hndlp (IN)
        // The error handle, usually, or the environment
        // handle (for errors on OCIEnvCreate(), OCIHandleAlloc()).
        hndlp: *mut c_void,

        // recordno (IN)
        // Indicates the status record from which the application seeks information.
        // Starts from 1.
        recordno: c_uint,

        // sqlstate (OUT)
        // Not supported in release 8.x or later.
        sqlstate: *mut c_uchar,

        // errcodep (OUT)
        // The error code returned.
        errcodep: *mut c_int,

        // bufp (OUT)
        // The error message text returned.
        bufp: *mut c_uchar,

        // bufsiz (IN)
        // The size of the buffer provided for the error message, in number of bytes.
        // If the error message length is more than bufsiz, a truncated error
        // message text is returned in bufp.
        // 
        // If type is set to OCI_HTYPE_ERROR, then the return code during truncation
        // for OCIErrorGet() is OCI_ERROR. The client can then specify a bigger
        // buffer and call OCIErrorGet() again.
        // 
        // If bufsiz is sufficient to hold the entire message text and the message could be
        // successfully copied into bufp, the return code for OCIErrorGet() is OCI_SUCCESS.
        bufsiz: c_uint,

        // type (IN)
        // The type of the handle (OCI_HTYPE_ERROR or OCI_HTYPE_ENV).
        _type: c_uint
    ) -> c_int;

    // Sets the value of an attribute of a handle or a descriptor
    fn OCIAttrSet(
        // trgthndlp (IN/OUT)
        // Pointer to a handle whose attribute gets modified.
        trgthndlp: *mut c_void,

        // trghndltyp (IN/OUT)
        // The handle type.
        trghndltyp: c_uint,

        // attributep (IN)
        // Pointer to an attribute value. The attribute value is copied into the target handle.
        // If the attribute value is a pointer, then only the pointer is copied, not the
        // contents of the pointer. String attributes must be in the encoding specified by the
        // charset parameter of a previous call to OCIEnvNlsCreate().
        attributep: *mut c_void,

        // size (IN)
        // The size of an attribute value. This can be passed in as 0 for most attributes,
        // as the size is already known by the OCI library. For text* attributes,
        // a ub4 (c_uint) must be passed in set to the length of the
        // string in bytes, regardless of encoding.
        size: c_uint,

        // attrtype (IN)
        // The type of attribute being set.
        attrtype: c_uint,

        // errhp (IN/OUT)
        // An error handle that you can pass to OCIErrorGet() for diagnostic
        // information when there is an error.
        errhp: *mut OCIError
    ) -> c_int;

    // Creates a user session and begins a user session for a given server.
    // The OCISessionBegin() call is used to authenticate a user against the server set in the
    // service context handle.
    // 
    // Check for any errors returned when trying to start a session. For example, if the password
    // for the account has expired, an ORA-28001 error is returned.
    // 
    // For release 8.1 or later, OCISessionBegin() must be called for any given server handle
    // before requests can be made against it. OCISessionBegin() only supports authenticating the
    // user for access to the Oracle database specified by the server handle in the service context.
    // In other words, after OCIServerAttach() is called to initialize a server handle,
    // OCISessionBegin() must be called to authenticate the user for that given server.
    // 
    // When using Unicode, when the mode or the environment handle has the appropriate setting, the
    // user name and password that have been set in the session handle usrhp should be in Unicode.
    // Before calling this function to start a session with a user name and password, you must have
    // called OCIAttrSet() to set these two Unicode strings into the session handle with
    // corresponding length in bytes, because OCIAttrSet() only takes void pointers.
    // The string buffers then are interpreted by OCISessionBegin().
    // 
    // When OCISessionBegin() is called for the first time for a given server handle, the user
    // session may not be created in migratable (OCI_MIGRATE) mode.
    // 
    // After OCISessionBegin() has been called for a server handle, the application may call
    // OCISessionBegin() again to initialize another user session handle with
    // different (or the same) credentials and different (or the same) operation modes. If an
    // application wants to authenticate a user in OCI_MIGRATE mode, the service handle must be
    // associated with a nonmigratable user handle. The user ID of that user handle becomes the
    // ownership ID of the migratable user session. Every migratable session must have a
    // nonmigratable parent session.
    // 
    // If the OCI_MIGRATE mode is not specified, then the user session context can only be used
    // with the same server handle set in svchp. If the OCI_MIGRATE mode is specified, then the
    // user authentication can be set with different server handles. However, the user session
    // context can only be used with server handles that resolve to the same database instance.
    // Security checking is done during session switching. A session can migrate to another process
    // only if there is a nonmigratable session currently connected to that process whose userid is
    // the same as that of the creator's userid or its own userid.
    // 
    // Do not set the OCI_MIGRATE flag in the call to OCISessionBegin() when the virtual server
    // handle points to a connection pool (OCIServerAttach() called with mode set to OCI_CPOOL).
    // Oracle Database supports passing this flag only for compatibility reasons. Do not use
    // the OCI_MIGRATE flag, as the perception that the user gets when using a connection pool is
    // of sessions having their own dedicated (virtual) connections that are transparently
    // multiplexed onto real connections.
    // 
    // OCI_SYSDBA, OCI_SYSOPER, and OCI_PRELIM_AUTH can only be used with a
    // primary user session context.
    // 
    // To provide credentials for a call to OCISessionBegin(), two methods are supported. The first
    // method is to provide a valid user name and password pair for database authentication in the
    // user session handle passed to OCISessionBegin(). This involves using OCIAttrSet() to set the
    // OCI_ATTR_USERNAME and OCI_ATTR_PASSWORD attributes on the user session handle. Then
    // OCISessionBegin() is called with OCI_CRED_RDBMS.
    // 
    // When the user session handle is terminated using OCISessionEnd(), the user name and password
    // attributes remain unchanged and thus can be reused in a future call to OCISessionBegin().
    // Otherwise, they must be reset to new values before the next OCISessionBegin() call.
    // 
    // The second method is to use external credentials. No attributes need to be set on the user
    // session handle before calling OCISessionBegin(). The credential type is OCI_CRED_EXT.
    // This is equivalent to the Oracle7 'connect /' syntax. If values have been set for
    // OCI_ATTR_USERNAME and OCI_ATTR_PASSWORD, then these are ignored if OCI_CRED_EXT is used.
    // 
    // Another way of setting credentials is to use the session ID of an authenticated user with
    // the OCI_MIGSESSION attribute. This ID can be extracted from the session handle of an
    // authenticated user using the OCIAttrGet() call.
    fn OCISessionBegin(
        // svchp (IN)
        // A handle to a service context. There must be a valid server handle set in svchp.
        svchp: *mut OCISvcCtx,

        // errhp (IN)
        // An error handle that you can pass to OCIErrorGet() for diagnostic information when
        // there is an error.
        errhp: *mut OCIError,

        // usrhp (IN/OUT)
        // A handle to a user session context, which is initialized by this call.
        usrhp: *mut OCISession,

        // credt (IN)
        // Specifies the type of credentials to use for establishing the user session.
        // Valid values for credt are:
        //   OCI_CRED_RDBMS - Authenticate using a database user name and password pair as
        //     credentials. The attributes OCI_ATTR_USERNAME and OCI_ATTR_PASSWORD should be set on
        //     the user session context before this call.
        //   OCI_CRED_EXT - Authenticate using external credentials.
        //     No user name or password is provided.
        credt: c_uint,

        // mode (IN)
        // Specifies the various modes of operation. Valid modes are:
        //   OCI_DEFAULT - In this mode, the user session context returned can only ever be set
        //     with the server context specified in svchp. For encoding, the server handle uses the
        //     setting in the environment handle.
        //   OCI_MIGRATE - In this mode, the new user session context can be set in a service
        //     handle with a different server handle. This mode establishes the user session
        //     context. To create a migratable session, the service handle must already be set with
        //     a nonmigratable user session, which becomes the "creator" session of the migratable
        //     session. That is, a migratable session must have a nonmigratable parent session.
        //     OCI_MIGRATE should not be used when the session uses connection pool underneath.
        //     The session migration and multiplexing happens transparently to the user.
        //   OCI_SYSDBA - In this mode, the user is authenticated for SYSDBA access.
        //   OCI_SYSOPER - In this mode, the user is authenticated for SYSOPER access.
        //   OCI_PRELIM_AUTH - This mode can only be used with OCI_SYSDBA or OCI_SYSOPER to
        //     authenticate for certain administration tasks.
        //   OCI_STMT_CACHE - Enables statement caching with default size on the given service
        //     handle. It is optional to pass this mode if the application is going to explicitly
        //     set the size later using OCI_ATTR_STMTCACHESIZE on that service handle.
        mode: c_uint
    ) -> c_int;

    // Terminates a user session context created by OCISessionBegin()
    // The user security context associated with the service context is invalidated by this call.
    // Storage for the user session context is not freed. The transaction specified by the service
    // context is implicitly committed. The transaction handle, if explicitly allocated, may be
    // freed if it is not being used. Resources allocated on the server for this user are freed.
    // The user session handle can be reused in a new call to OCISessionBegin().
    fn OCISessionEnd(
        // svchp (IN/OUT)
        // The service context handle. There must be a valid server handle and user session
        // handle associated with svchp.
        svchp: *mut OCISvcCtx,

        // errhp (IN/OUT)
        // An error handle that you can pass to OCIErrorGet() for diagnostic information
        // when there is an error.
        errhp: *mut OCIError,

        // usrhp (IN)
        // Deauthenticate this user. If this parameter is passed as NULL, the user in the
        // service context handle is deauthenticated.
        usrhp: *mut OCISession,

        // mode (IN)
        // The only valid mode is OCI_DEFAULT.
        mode: c_uint
    ) -> c_int;

    // Deletes an access path to a data source for OCI operations.
    // This call deletes an access path a to data source for OCI operations.
    // The access path was established by a call to OCIServerAttach().
    fn OCIServerDetach(
        // srvhp (IN)
        // A handle to an initialized server context, which is reset to an uninitialized state.
        // The handle is not deallocated.
        srvhp: *mut OCIServer,

        // errhp (IN/OUT)
        // An error handle that you can pass to OCIErrorGet() for diagnostic information
        // when there is an error.
        errhp: *mut OCIError,

        // mode (IN)
        // Specifies the various modes of operation. The only valid mode is OCI_DEFAULT
        // for the default mode.
        mode: c_uint
    ) -> c_int;

    // This call explicitly deallocates a handle.
    // This call frees up storage associated with a handle, corresponding
    // to the type specified in the type parameter.
    // This call returns either OCI_SUCCESS, OCI_INVALID_HANDLE, or OCI_ERROR.
    // All handles may be explicitly deallocated. The OCI deallocates a
    // child handle if the parent is deallocated.
    // When a statement handle is freed, the cursor associated with the statement handle is closed, 
    // but the actual cursor closing may be deferred to the next round-trip to the server.
    // If the application must close the cursor immediately, you can make a server round-trip call,
    // such as OCIServerVersion() or OCIPing(), after the OCIHandleFree() call.
    fn OCIHandleFree(
        // hndlp (IN)
        // A handle allocated by OCIHandleAlloc().
        hndlp: *mut c_void,

        // type (IN)
        // Specifies the type of storage to be freed.
        _type: c_uint
    ) -> c_int;
}

pub fn oci_env_nls_create(mode: OCIMode) -> Result<*mut OCIEnv, OracleError> {
    let mut handle = ptr::null_mut();
    let res = unsafe {
        OCIEnvNlsCreate(
            &mut handle,     // envp
            mode as c_uint,  // mode
            ptr::null_mut(), // ctxp
            None,            // malocfp
            None,            // ralocfp
            None,            // mfreefp
            0,               // xtramem_sz
            ptr::null_mut(), // usrmempp
            0,               // charset
            0                // ncharset
        )
    };
    match check_error(res, None, "ffi::oci_env_nls_create") {
        None      => Ok(handle),
        Some(err) => Err(err),
    }
}

pub fn oci_handle_alloc(envh: *mut OCIEnv,
                        htype: OCIHandleType) -> Result<*mut c_void, OracleError> {
    let mut handle = ptr::null_mut();
    let res = unsafe {
        OCIHandleAlloc(
            envh as *const _, // parenth
            &mut handle,      // hndlpp
            htype as c_uint,  // type
            0,                // xtramem_sz
            ptr::null_mut()   // usrmempp
        )
    };
    match check_error(res, None, "ffi::oci_handle_alloc") {
        None => Ok(handle),
        Some(err) => Err(err),
    }
}

pub fn oci_server_attach(server_handle: *mut OCIServer,
                         error_handle: *mut OCIError,
                         db: String,
                         mode: OCIMode) -> Result<(), OracleError> {
    let res = db.with_c_str(|s|
        unsafe {
            OCIServerAttach(
                server_handle,       // srvhp
                error_handle,        // errhp
                s as *const c_uchar, // dblink
                db.len() as c_int,   // dblink_len
                mode as c_uint       // mode
            )
        }
    );
    match check_error(res, Some(error_handle), "ffi::oci_server_attach") {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

pub fn oci_error_get(error_handle: *mut OCIError, location: &str) -> OracleError {
    let errc: *mut int = &mut 0;
    let buf = String::with_capacity(3072);
    let msg = buf.with_c_str(|errm|
        unsafe {
            OCIErrorGet(
                error_handle as *mut c_void,   // hndlp
                1,                             // recordno
                ptr::null_mut(),               // sqlstate
                errc as *mut c_int,            // errcodep
                errm as *mut c_uchar,          // bufp
                buf.capacity() as c_uint,      // bufsiz
                OCIHandleType::Error as c_uint // type
            );
            match CString::new(errm, false).as_str() {
                Some(s) => s.trim().to_string(),
                None    => String::new(),
            }
        }
    );
    OracleError {code: unsafe { *errc }, message: msg, location: location.to_string()}
}

pub fn oci_attr_set(handle: *mut c_void,
                    htype: OCIHandleType,
                    value: *mut c_void,
                    attr_type: OCIAttribute,
                    error_handle: *mut OCIError) -> Result<(), OracleError> {
    let size: c_uint = match attr_type {
        OCIAttribute::Username | OCIAttribute::Password => unsafe {
            CString::new(value as *const c_char, false).len() as c_uint
        },
        _ => 0,
    };
    let res = unsafe {
        OCIAttrSet(
            handle,              // trgthndlp
            htype as c_uint,     // trghndltyp
            value,               // attributep
            size,                // size
            attr_type as c_uint, // attrtype
            error_handle         // errhp
        )
    };
    match check_error(res, Some(error_handle), "ffi::oci_attr_set") {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

pub fn oci_session_begin(service_handle: *mut OCISvcCtx,
                         error_handle: *mut OCIError,
                         session_handle: *mut OCISession,
                         credentials_type: OCICredentialsType,
                         mode: OCIAuthMode) -> Result<(), OracleError> {
    let res = unsafe {
        OCISessionBegin(
            service_handle,             // svchp
            error_handle,               // errhp
            session_handle,             // usrhp
            credentials_type as c_uint, // credt
            mode as c_uint              // mode
        )
    };
    match check_error(res, Some(error_handle), "ffi::oci_session_begin") {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

pub fn oci_session_end(service_handle: *mut OCISvcCtx,
                       error_handle: *mut OCIError,
                       session_handle: *mut OCISession) -> Result<(), OracleError> {
    let res = unsafe {
        OCISessionEnd(
            service_handle,                // svchp
            error_handle,                  // errhp
            session_handle,                // usrhp
            OCIAuthMode::Default as c_uint // mode
        )
    };
    match check_error(res, Some(error_handle), "ffi::oci_session_end") {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

pub fn oci_server_detach(server_handle: *mut OCIServer,
                         error_handle: *mut OCIError) -> Result<(), OracleError> {
    let res = unsafe {
        OCIServerDetach(server_handle, error_handle, OCIMode::Default as c_uint)
    };
    match check_error(res, Some(error_handle), "ffi::oci_server_detach") {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

pub fn oci_handle_free(handle: *mut c_void, htype: OCIHandleType) -> Result<(), OracleError> {
    let res = unsafe {
        OCIHandleFree(handle, htype as c_uint)
    };
    match check_error(res, None, "ffi::oci_handle_free") {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

pub fn check_error(code: c_int,
                   error_handle: Option<*mut OCIError>,
                   location: &str) -> Option<OracleError> {
    let by_handle = match error_handle {
        Some(handle) => Some(oci_error_get(handle, location)),
        None         => None,
    };
    match code {
        0     => None,
        100   => Some(OracleError {
            code: code as int, message: "No data".to_string(), location: location.to_string()
        }),
        -2    => Some(OracleError {
            code: code as int, message: "Invalid handle".to_string(), location: location.to_string()
        }),
        99    => Some(OracleError {
            code: code as int, message: "Need data".to_string(), location: location.to_string()
        }),
        -3123 => Some(OracleError {
            code: code as int, message: "Still executing".to_string(),
            location: location.to_string()
        }),
        -1    => Some(by_handle.unwrap_or(OracleError {
            code: code as int, message: "Error with no details".to_string(),
            location: location.to_string()
        })),
        1     => Some(by_handle.unwrap_or(OracleError {
            code: code as int, message: "Success with info".to_string(),
            location: location.to_string()
        })),
        _     => panic!("Unknown return code"),
    }
}
