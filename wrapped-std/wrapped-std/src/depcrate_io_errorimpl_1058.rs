// Generated macro for impl_1058 (impl)
macro_rules! Depcrate_io_errorimpl_1058 {
() => {
// Module: crate::io::error
// Provides: {"impl_1058"}
// Dependencies: {}
# [doc = " Common errors constants for use in std"] # [allow (dead_code)] impl Error { pub (crate) const INVALID_UTF8 : Self = const_error ! (ErrorKind :: InvalidData , "stream did not contain valid UTF-8") ; pub (crate) const READ_EXACT_EOF : Self = const_error ! (ErrorKind :: UnexpectedEof , "failed to fill whole buffer") ; pub (crate) const UNKNOWN_THREAD_COUNT : Self = const_error ! (ErrorKind :: NotFound , "the number of hardware threads is not known for the target platform" ,) ; pub (crate) const UNSUPPORTED_PLATFORM : Self = const_error ! (ErrorKind :: Unsupported , "operation not supported on this platform") ; pub (crate) const WRITE_ALL_EOF : Self = const_error ! (ErrorKind :: WriteZero , "failed to write whole buffer") ; pub (crate) const ZERO_TIMEOUT : Self = const_error ! (ErrorKind :: InvalidInput , "cannot set a 0 duration timeout") ; pub (crate) const NO_ADDRESSES : Self = const_error ! (ErrorKind :: InvalidInput , "could not resolve to any addresses") ; }
};
}
