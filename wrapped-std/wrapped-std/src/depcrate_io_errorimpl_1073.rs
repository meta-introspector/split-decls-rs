// Generated macro for impl_1073 (impl)
macro_rules! Depcrate_io_errorimpl_1073 {
() => {
// Module: crate::io::error
// Provides: {"impl_1073"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl error :: Error for Error { # [allow (deprecated)] fn cause (& self) -> Option < & dyn error :: Error > { match self . repr . data () { ErrorData :: Os (..) => None , ErrorData :: Simple (..) => None , ErrorData :: SimpleMessage (..) => None , ErrorData :: Custom (c) => c . error . cause () , } } fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match self . repr . data () { ErrorData :: Os (..) => None , ErrorData :: Simple (..) => None , ErrorData :: SimpleMessage (..) => None , ErrorData :: Custom (c) => c . error . source () , } } }
};
}
