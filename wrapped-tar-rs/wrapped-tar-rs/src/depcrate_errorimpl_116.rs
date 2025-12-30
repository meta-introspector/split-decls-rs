// Generated macro for impl_116 (impl)
macro_rules! Depcrate_errorimpl_116 {
() => {
// Module: crate::error
// Provides: {"impl_116"}
// Dependencies: {}
impl From < TarError > for Error { fn from (t : TarError) -> Error { Error :: new (t . io . kind () , t) } }
};
}
