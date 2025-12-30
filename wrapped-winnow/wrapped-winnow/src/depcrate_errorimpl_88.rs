// Generated macro for impl_88 (impl)
macro_rules! Depcrate_errorimpl_88 {
() => {
// Module: crate::error
// Provides: {"impl_88"}
// Dependencies: {}
# [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] impl < I : Stream > FromRecoverableError < I , Self > for EmptyError { # [inline (always)] fn from_recoverable_error (_token_start : & < I as Stream > :: Checkpoint , _err_start : & < I as Stream > :: Checkpoint , _input : & I , e : Self ,) -> Self { e } }
};
}
