// Generated macro for impl_123 (impl)
macro_rules! Depcrate_errorimpl_123 {
() => {
// Module: crate::error
// Provides: {"impl_123"}
// Dependencies: {}
# [cfg (feature = "std")] # [cfg (feature = "unstable-recover")] impl < I : Stream , C > FromRecoverableError < I , Self > for TreeError < I , C > { # [inline] fn from_recoverable_error (_token_start : & < I as Stream > :: Checkpoint , _err_start : & < I as Stream > :: Checkpoint , _input : & I , e : Self ,) -> Self { e } }
};
}
