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

#[repr(C)]
struct OCISnapshot;

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

enum OCISyntax {
    NtvSyntax = 1, // OCI_NTV_SYNTAX
}

enum OCIStmtPrepare2Mode {
    Default = 0x00000000, // OCI_DEFAULT
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

#[allow(dead_code)]
enum OCIDescriptorType {
    Parameter = 53, // OCI_DTYPE_PARAM
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

    // Prepares a SQL or PL/SQL statement for execution. The user has the option of using the
    // statement cache, if it has been enabled.
    // 
    // An OCI application uses this call to prepare a SQL or PL/SQL statement for execution.
    // The OCIStmtPrepare2() call defines an application request.
    // 
    // The mode parameter determines whether the statement content is encoded as UTF-16 or not.
    // The statement length is in number of code points or in number of bytes,
    // depending on the encoding.
    // 
    // Although the statement handle inherits the encoding setting from the parent environment
    // handle, the mode for this call can also change the encoding setting
    // for the statement handle itself.
    // 
    // Data values for this statement initialized in subsequent bind calls are stored in a bind
    // handle that uses settings in this statement handle as the default.
    // 
    // This call does not create an association between this
    // statement handle and any particular server.
    // 
    // Before reexecuting a DDL statement, call this function a second time.
    fn OCIStmtPrepare2(
        // svchp (IN)
        // The service context to be associated with the statement.
        svchp: *mut OCISvcCtx,

        // stmtp (OUT)
        // Pointer to the statement handle returned.
        stmtp: *mut *mut OCIStmt,

        // errhp (IN)
        // A pointer to the error handle for diagnostics.
        errhp: *mut OCIError,

        // stmttext (IN)
        // The statement text. SQL or PL/SQL statement to be executed.
        // Must be a NULL-terminated string. That is, the ending character is a number of NULL
        // bytes, depending on the encoding. The statement must be in the encoding specified by the
        // charset parameter of a previous call to OCIEnvNlsCreate().
        // Always cast the parameter to (text *). After a statement has been prepared in UTF-16,
        // the character set for the bind and define buffers default to UTF-16.
        stmt: *const c_uchar,

        // stmt_len (IN)
        // The statement text length.
        stmt_len: c_uint,

        // key (IN)
        // For statement caching only. The key to be used for searching the statement in the
        // statement cache. If the key is passed in, then the statement text and other parameters
        // are ignored, and the search is based solely on the key.
        key: *const c_uchar,

        // key_len (IN)
        // For statement caching only. The length of the key.
        key_len: c_uint,

        // language (IN)
        // Specifies V7, or native syntax. Possible values are as follows:
        //   OCI_V7_SYNTAX - V7 ORACLE parsing syntax.
        //   OCI_NTV_SYNTAX - Syntax depends upon the version of the server.
        language: c_uint,

        // mode (IN)
        // This function can be used with and without statement caching. This is determined at the
        // time of connection or session pool creation. If caching is enabled for a session, then
        // all statements in the session have caching enabled, and if caching is not enabled,
        // then all statements are not cached.
        // The valid modes are as follows:
        //   OCI_DEFAULT - Caching is not enabled. This is the only valid setting. If the statement
        //   is not found in the cache, this mode allocates a new statement handle and prepares the
        //   statement handle for execution. If the statement is not found in the cache and one of
        //   the following circumstances applies, then the subsequent actions follow:
        //     Only the text has been supplied: a new statement is allocated and prepared and
        //       returned. The tag NULL. OCI_SUCCESS is returned.
        //     Only the tag has been supplied: stmthp is NULL. OCI_ERROR is returned.
        //     Both text and key were supplied: a new statement is allocated and prepared and
        //       returned. The tag NULL. OCI_SUCCESS_WITH_INFO is returned, as the returned
        //       statement differs from the requested statement in that the tag is NULL.
        //   OCI_PREP2_CACHE_SEARCHONLY - In this case, if the statement is not found (a NULL
        //     statement handle is returned), you must take further action. If the statement is
        //     found, OCI_SUCCESS is returned. Otherwise, OCI_ERROR is returned.
        //   OCI_PREP2_GET_PLSQL_WARNINGS - If warnings are enabled in the session and the PL/SQL
        //     program is compiled with warnings, then OCI_SUCCESS_WITH_INFO is the return status
        //     from the execution. Use OCIErrorGet() to find the new error number corresponding
        //     to the warnings.
        mode: c_uint
    ) -> c_int;

    // Associates an application request with a server.
    // This function is used to execute a prepared SQL statement. Using an execute call, the
    // application associates a request with a server.
    // 
    // If a SELECT statement is executed, the description of the select list is available
    // implicitly as a response. This description is buffered on the client side for describes,
    // fetches, and define type conversions. Hence it is optimal to describe a select list only
    // after an execute.
    // 
    // Also for SELECT statements, some results are available implicitly. Rows are received and
    // buffered at the end of the execute. For queries with small row count, a prefetch causes
    // memory to be released in the server if the end of fetch is reached, an optimization that may
    // result in memory usage reduction. The set attribute call has been defined to set the number
    // of rows to be prefetched for each result set.
    // 
    // For SELECT statements, at the end of the execute, the statement handle implicitly maintains
    // a reference to the service context on which it is executed. It is the user's responsibility
    // to maintain the integrity of the service context. The implicit reference is maintained until
    // the statement handle is freed or the fetch is canceled or an end of
    // fetch condition is reached.
    // 
    // To reexecute a DDL statement, you must prepare the statement again
    // using OCIStmtPrepare() or OCIStmtPrepare2().
    // 
    // If output variables are defined for a SELECT statement before a call to OCIStmtExecute(),
    // the number of rows specified by iters are fetched directly into the defined output buffers
    // and additional rows equivalent to the prefetch count are prefetched. If there are no
    // additional rows, then the fetch is complete without calling OCIStmtFetch2() or
    // deprecated OCIStmtFetch().
    fn OCIStmtExecute(
        // svchp (IN/OUT)
        // Service context handle.
        svchp: *mut OCISvcCtx,

        // stmtp (IN/OUT)
        // A statement handle. It defines the statement and the associated data to be executed at
        // the server. It is invalid to pass in a statement handle that has bind of data types only
        // supported in release 8.x or later when svchp points to an Oracle7 server.
        stmtp: *mut OCIStmt,

        // errhp (IN/OUT)
        // An error handle that you can pass to OCIErrorGet() for diagnostic information when
        // there is an error.
        errhp: *mut OCIError,

        // iters (IN)
        // For non-SELECT statements, the number of times this statement
        // is executed equals iters - rowoff.
        // For SELECT statements, if iters is nonzero, then defines must have been done for the
        // statement handle. The execution fetches iters rows into these predefined buffers and
        // prefetches more rows depending upon the prefetch row count. If you do not know how many
        // rows the SELECT statement retrieves, set iters to zero.
        // This function returns an error if iters=0 for non-SELECT statements.
        // For array DML operations, set iters <= 32767 to get better performance.
        iters: c_uint,

        // rowoff (IN)
        // The starting index from which the data in an array bind is relevant for this
        // multiple row execution.
        rowoff: c_uint,

        // snap_in (IN)
        // This parameter is optional. If it is supplied, it must point to a snapshot descriptor of
        // type OCI_DTYPE_SNAP. The contents of this descriptor must be obtained from the snap_out
        // parameter of a previous call. The descriptor is ignored if the SQL is not a SELECT
        // statement. This facility allows multiple service contexts to Oracle Database to see the
        // same consistent snapshot of the database's committed data. However, uncommitted data in
        // one context is not visible to another context even using the same snapshot.
        snap_in: *const OCISnapshot,

        // snap_out (OUT)
        // This parameter is optional. If it is supplied, it must point to a descriptor of type
        // OCI_DTYPE_SNAP. This descriptor is filled in with an opaque representation that is the
        // current Oracle Database system change number (SCN) suitable as a snap_in input to a
        // subsequent call to OCIStmtExecute(). To avoid "snapshot too old" errors, do not use this
        // descriptor any longer than necessary.
        snap_out: *mut OCISnapshot,

        // The modes are:
        //   OCI_BATCH_ERRORS - See "Batch Error Mode" for information about this mode.
        //   OCI_COMMIT_ON_SUCCESS - When a statement is executed in this mode, the current
        //     transaction is committed after execution, if execution completes successfully.
        //   OCI_DEFAULT - Calling OCIStmtExecute() in this mode executes the statement. It also
        //     implicitly returns describe information about the select list.
        //   OCI_DESCRIBE_ONLY - This mode is for users who want to describe a query before
        //     execution. Calling OCIStmtExecute() in this mode does not execute the statement, but
        //     it does return the select-list description. To maximize performance, Oracle
        //     recommends that applications execute the statement in default mode and use the
        //     implicit describe that accompanies the execution.
        //   OCI_EXACT_FETCH - Used when the application knows in advance exactly how many rows it
        //     is fetching. This mode turns prefetching off for Oracle Database release 8 or later
        //     mode, and requires that defines be done before the execute call. Using this mode
        //     cancels the cursor after the desired rows are fetched and may result in
        //     reduced server-side resource usage.
        //   OCI_PARSE_ONLY - This mode allows the user to parse the query before execution.
        //     Executing in this mode parses the query and returns parse errors in the SQL, if any.
        //     Users must note that this involves an additional round-trip to the server. To
        //     maximize performance, Oracle recommends that the user execute the statement in the
        //     default mode, which, parses the statement as part of the bundled operation.
        //   OCI_STMT_SCROLLABLE_READONLY - Required for the result set to be scrollable. The
        //     result set cannot be updated. See "Fetching Results".
        //     This mode cannot be used with any other mode.
        // The modes are not mutually exclusive; you can use them together,
        // except for OCI_STMT_SCROLLABLE_READONLY.
        mode: c_uint
    ) -> c_int;

    // Releases the statement handle obtained by a call to OCIStmtPrepare2().
    fn OCIStmtRelease(
        // stmtp (IN/OUT)
        // The statement handle returned by OCIStmtPrepare2()
        stmtp: *mut OCIStmt,

        // errhp (IN)
        // The error handle used for diagnostics.
        errhp: *mut OCIError,

        // key (IN)
        // Only valid for statement caching. The key to be associated with the statement in the
        // cache. This is a SQL string passed in by the caller. If a NULL key is passed in,
        // the statement is not tagged.
        key: *const c_uchar,

        // keylen (IN)
        // Only valid for statement caching. The length of the key.
        key_len: c_uint,

        // mode (IN)
        // The valid modes are:
        //   OCI_DEFAULT
        //   OCI_STRLS_CACHE_DELETE - Only valid for statement caching. The statement is not
        //     kept in the cache anymore.
        mode: c_uint
    ) -> c_int;

    // This call returns a descriptor of a parameter specified by position in the describe handle
    // or statement handle. Parameter descriptors are always allocated internally by the OCI
    // library. They can be freed using OCIDescriptorFree(). For example, if you fetch the same
    // column metadata for every execution of a statement, then the program leaks memory unless you
    // explicitly free the parameter descriptor between each call to OCIParamGet().
    fn OCIParamGet(
        // hndlp (IN)
        // A statement handle or describe handle. The OCIParamGet() function returns a
        // parameter descriptor for this handle.
        hndlp: *const c_void,

        // htype (IN)
        // The type of the handle passed in the hndlp parameter. Valid types are:
        //   OCI_DTYPE_PARAM, for a parameter descriptor
        //   OCI_HTYPE_COMPLEXOBJECT, for a complex object retrieval handle
        //   OCI_HTYPE_STMT, for a statement handle
        htype: c_uint,

        // errhp (IN/OUT)
        // An error handle that you can pass to OCIErrorGet() for diagnostic information when
        // there is an error.
        errhp: *mut OCIError,

        // parmdpp (OUT)
        // A descriptor of the parameter at the position given in the pos parameter,
        // of handle type OCI_DTYPE_PARAM.
        parmdpp: *mut *mut c_void,

        // pos (IN)
        // Position number in the statement handle or describe handle. A parameter descriptor is
        // returned for this position.
        // OCI_ERROR is returned if there are no parameter descriptors for this position.
        pos: c_uint
    ) -> c_int;

    // Gets the value of an attribute of a handle.
    // This call is used to get a particular attribute of a handle. OCI_DTYPE_PARAM is used to do
    // implicit and explicit describes. The parameter descriptor is also used in direct path
    // loading. For implicit describes, the parameter descriptor has the column description for
    // each select list. For explicit describes, the parameter descriptor has the describe
    // information for each schema object that you are trying to describe. If the top-level
    // parameter descriptor has an attribute that is itself a descriptor, use OCI_ATTR_PARAM as the
    // attribute type in the subsequent call to OCIAttrGet() to get the Unicode information in an
    // environment or statement handle.
    // 
    // A function closely related to OCIAttrGet() is OCIDescribeAny(), which is a generic describe
    // call that describes existing schema objects: tables, views, synonyms, procedures, functions,
    // packages, sequences, and types. As a result of this call, the describe handle is populated
    // with the object-specific attributes that can be obtained through an OCIAttrGet() call.
    // 
    // Then an OCIParamGet() call on the describe handle returns a parameter descriptor for a
    // specified position. Parameter positions begin with 1. Calling OCIAttrGet() on the parameter
    // descriptor returns the specific attributes of a stored procedure or function parameter or a
    // table column descriptor. These subsequent calls do not need an extra round-trip to the
    // server because the entire schema object description is cached on the client side by
    // OCIDescribeAny(). Calling OCIAttrGet() on the describe handle can also return the total
    // number of positions.
    // 
    // In UTF-16 mode, particularly when executing a loop, try to reuse the same pointer variable
    // corresponding to an attribute and copy the contents to local variables after OCIAttrGet() is
    // called. If multiple pointers are used for the same attribute, a memory leak can occur.
    fn OCIAttrGet(
        // trgthndlp (IN)
        // Pointer to a handle type. The actual handle can be a statement handle, a session handle,
        // and so on. When this call is used to get encoding, users are allowed to check against
        // either an environment or statement handle.
        trgthndlp: *const c_void,

        // trghndltyp (IN)
        // The handle type. Valid types are:
        //   OCI_DTYPE_PARAM, for a parameter descriptor
        //   OCI_HTYPE_STMT, for a statement handle
        //   Any handle type in OCIHandleType enum or any descriptor in OCIDescriptorType enum.
        trghndltyp: c_uint,

        // attributep (OUT)
        // Pointer to the storage for an attribute value. Is in the encoding specified by the
        // charset parameter of a previous call to OCIEnvNlsCreate().
        attributep: *mut c_void,

        // sizep (OUT)
        // The size of the attribute value, always in bytes because attributep is a void pointer.
        // This can be passed as NULL for most attributes because the sizes of non-string
        // attributes are already known by the OCI library. For text* parameters, a pointer to a
        // ub4 must be passed in to get the length of the string.
        sizep: *mut c_uint,

        // attrtype (IN)
        // The type of attribute being retrieved.
        attrtype: c_uint,

        // errhp (IN/OUT)
        // An error handle that you can pass to OCIErrorGet() for diagnostic information when
        // there is an error.
        errhp: *mut OCIError
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
    let res = db.with_c_str(|s| unsafe {
        OCIServerAttach(
            server_handle,       // srvhp
            error_handle,        // errhp
            s as *const c_uchar, // dblink
            db.len() as c_int,   // dblink_len
            mode as c_uint       // mode
        )
    });
    match check_error(res, Some(error_handle), "ffi::oci_server_attach") {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

pub fn oci_error_get(error_handle: *mut OCIError, location: &str) -> OracleError {
    let errc: *mut int = &mut 0;
    let buf = String::with_capacity(3072);
    let msg = buf.with_c_str(|errm| unsafe {
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
    });
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

pub fn oci_stmt_prepare2(service_handle: *mut OCISvcCtx,
                         error_handle: *mut OCIError,
                         stmt_text: &String,
                         stmt_hash: &String) -> Result<*mut OCIStmt, OracleError> {
    let mut stmt_handle = ptr::null_mut();
    let res = stmt_text.with_c_str(|stmt|
        stmt_hash.with_c_str(|hash| unsafe {
            OCIStmtPrepare2(
                service_handle,                        // svchp
                &mut stmt_handle,                      // stmtp
                error_handle,                          // errhp
                stmt as *const c_uchar,                // stmttext
                stmt_text.len() as c_uint,             // stmt_len
                hash as *const c_uchar,                // key
                stmt_hash.len() as c_uint,             // key_len
                OCISyntax::NtvSyntax as c_uint,        // language
                OCIStmtPrepare2Mode::Default as c_uint // mode
            )
        })
    );
    match check_error(res, Some(error_handle), "ffi::oci_stmt_prepare2") {
        None => Ok(stmt_handle),
        Some(err) => Err(err),
    }
}

pub fn oci_stmt_execute(service_handle: *mut OCISvcCtx,
                        stmt_handle: *mut OCIStmt,
                        error_handle: *mut OCIError) -> Result<(), OracleError> {
    let res = unsafe {
        OCIStmtExecute(
            service_handle,            // svchp
            stmt_handle,               // stmtp
            error_handle,              // errhp
            0 as c_uint,               // iters
            0 as c_uint,               // rowoff
            ptr::null(),               // snap_in
            ptr::null_mut(),           // snap_out
            OCIMode::Default as c_uint // mode
        )
    };
    match check_error(res, Some(error_handle), "ffi::oci_stmt_execute") {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

pub fn oci_stmt_release(stmt_handle: *mut OCIStmt,
                        error_handle: *mut OCIError,
                        stmt_hash: &String) -> Result<(), OracleError> {
    let res = stmt_hash.with_c_str(|hash| unsafe {
        OCIStmtRelease(
            stmt_handle,               // stmtp
            error_handle,              // errhp
            hash as *const c_uchar,    // key
            stmt_hash.len() as c_uint, // keylen
            OCIMode::Default as c_uint // mode
        )
    });
    match check_error(res, Some(error_handle), "ffi::oci_stmt_release") {
        None => Ok(()),
        Some(err) => Err(err),
    }
}

pub fn oci_param_get(stmt_handle: *mut OCIStmt,
                     error_handle: *mut OCIError,
                     position: uint) -> Result<*mut c_void, OracleError> {
    let mut parameter_descriptor = ptr::null_mut();
    let res = unsafe {
        OCIParamGet(
            stmt_handle as *const _,            // hndlp
            OCIHandleType::Statement as c_uint, // htype
            error_handle,                       // errhp
            &mut parameter_descriptor,          // parmdpp
            position as c_uint                  // pos
        )
    };
    match check_error(res, Some(error_handle), "ffi::oci_param_get") {
        None => Ok(parameter_descriptor),
        Some(err) => Err(err),
    }
}

pub fn oci_attr_get(attr_handle: *mut c_void,
                    error_handle: *mut OCIError,
                    attr_type: OCIAttribute) -> Result<(*mut c_void, int), OracleError> {
    let attribute = ptr::null_mut();
    let mut attribute_size = 0;
    let res = unsafe {
        OCIAttrGet(
            attr_handle as *const _,                // trgthndlp
            OCIDescriptorType::Parameter as c_uint, // trghndltyp
            attribute,                              // attributep
            &mut attribute_size,                    // sizep
            attr_type as c_uint,                    // attrtype
            error_handle                            // errhp
        )
    };
    match check_error(res, Some(error_handle), "ffi::oci_attr_get") {
        None => Ok((attribute, attribute_size as int)),
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
