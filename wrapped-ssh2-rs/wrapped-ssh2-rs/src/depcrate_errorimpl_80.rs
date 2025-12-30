// Generated macro for impl_80 (impl)
macro_rules! Depcrate_errorimpl_80 {
() => {
// Module: crate::error
// Provides: {"impl_80"}
// Dependencies: {}
impl From < Error > for io :: Error { fn from (err : Error) -> io :: Error { let kind = match err . code { ErrorCode :: Session (raw :: LIBSSH2_ERROR_EAGAIN) => io :: ErrorKind :: WouldBlock , ErrorCode :: Session (raw :: LIBSSH2_ERROR_TIMEOUT) => io :: ErrorKind :: TimedOut , ErrorCode :: SFTP (raw :: LIBSSH2_FX_NO_SUCH_FILE) | ErrorCode :: SFTP (raw :: LIBSSH2_FX_NO_SUCH_PATH) => io :: ErrorKind :: NotFound , _ => io :: ErrorKind :: Other , } ; io :: Error :: new (kind , err . msg) } }
};
}
