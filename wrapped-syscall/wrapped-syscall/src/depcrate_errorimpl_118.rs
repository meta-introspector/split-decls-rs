// Generated macro for impl_118 (impl)
macro_rules! Depcrate_errorimpl_118 {
() => {
// Module: crate::error
// Provides: {"impl_118"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < Error > for std :: io :: Error { fn from (value : Error) -> Self { std :: io :: Error :: from_raw_os_error (value . errno) } }
};
}
