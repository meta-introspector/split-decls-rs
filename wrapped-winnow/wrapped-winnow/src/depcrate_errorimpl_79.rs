// Generated macro for impl_79 (impl)
macro_rules! Depcrate_errorimpl_79 {
() => {
// Module: crate::error
// Provides: {"impl_79"}
// Dependencies: {}
# [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] impl < I : Clone + Stream > FromRecoverableError < I , Self > for InputError < I > { # [inline] fn from_recoverable_error (_token_start : & < I as Stream > :: Checkpoint , _err_start : & < I as Stream > :: Checkpoint , _input : & I , e : Self ,) -> Self { e } }
};
}
