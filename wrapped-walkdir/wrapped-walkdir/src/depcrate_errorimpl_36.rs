// Generated macro for impl_36 (impl)
macro_rules! Depcrate_errorimpl_36 {
() => {
// Module: crate::error
// Provides: {"impl_36"}
// Dependencies: {}
impl error :: Error for Error { # [allow (deprecated)] fn description (& self) -> & str { match self . inner { ErrorInner :: Io { ref err , .. } => err . description () , ErrorInner :: Loop { .. } => "file system loop found" , } } fn cause (& self) -> Option < & dyn error :: Error > { self . source () } fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match self . inner { ErrorInner :: Io { ref err , .. } => Some (err) , ErrorInner :: Loop { .. } => None , } } }
};
}
