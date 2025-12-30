// Generated macro for macro_126 (macro)
macro_rules! Depcrate_sessionmacro_126 {
() => {
// Module: crate::session
// Provides: {"macro_126"}
// Dependencies: {}
bitflags ! { # [doc = " Flags which can be used with the session trace method to set"] # [doc = " the trace level."] # [derive (PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Clone , Copy)] pub struct TraceFlags : c_int { # [doc = " Authentication debugging"] const AUTH = raw :: LIBSSH2_TRACE_AUTH ; # [doc = " Connection layer debugging"] const CONN = raw :: LIBSSH2_TRACE_CONN ; # [doc = " Error debugging"] const ERROR = raw :: LIBSSH2_TRACE_ERROR ; # [doc = " Key exchange debugging"] const KEX = raw :: LIBSSH2_TRACE_KEX ; # [doc = " Public Key Debugging"] const PUBLICKEY = raw :: LIBSSH2_TRACE_PUBLICKEY ; # [doc = " SCP debugging"] const SCP = raw :: LIBSSH2_TRACE_SCP ; # [doc = " SFTP debugging"] const SFTP = raw :: LIBSSH2_TRACE_SFTP ; # [doc = " Socket low-level debugging"] const SOCKET = raw :: LIBSSH2_TRACE_SOCKET ; # [doc = " Transport layer debugging"] const TRANS = raw :: LIBSSH2_TRACE_TRANS ; } }
};
}
