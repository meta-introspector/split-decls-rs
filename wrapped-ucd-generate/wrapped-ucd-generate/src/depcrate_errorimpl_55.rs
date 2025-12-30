// Generated macro for impl_55 (impl)
macro_rules! Depcrate_errorimpl_55 {
() => {
// Module: crate::error
// Provides: {"impl_55"}
// Dependencies: {}
impl error :: Error for Error { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match * self { Error :: Io (ref err) => Some (err) , Error :: Clap (ref err) => Some (err) , _ => None , } } }
};
}
