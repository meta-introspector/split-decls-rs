// Generated macro for impl_103 (impl)
macro_rules! Depcrate_errorimpl_103 {
() => {
// Module: crate::error
// Provides: {"impl_103"}
// Dependencies: {}
# [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] impl < I : Stream , C > FromRecoverableError < I , Self > for ContextError < C > { # [inline] fn from_recoverable_error (_token_start : & < I as Stream > :: Checkpoint , _err_start : & < I as Stream > :: Checkpoint , _input : & I , e : Self ,) -> Self { e } }
};
}
