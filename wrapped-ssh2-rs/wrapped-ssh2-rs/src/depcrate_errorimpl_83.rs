// Generated macro for impl_83 (impl)
macro_rules! Depcrate_errorimpl_83 {
() => {
// Module: crate::error
// Provides: {"impl_83"}
// Dependencies: {}
impl From < NulError > for Error { fn from (_ : NulError) -> Error { Error :: new (ErrorCode :: Session (raw :: LIBSSH2_ERROR_INVAL) , "provided data contained a nul byte and could not be used \
             as as string" ,) } }
};
}
