// Generated macro for impl_64 (impl)
macro_rules! Depcrate_errorimpl_64 {
() => {
// Module: crate::error
// Provides: {"impl_64"}
// Dependencies: {}
# [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] impl < I : Stream , E1 : FromRecoverableError < I , E2 > , E2 > FromRecoverableError < I , ErrMode < E2 > > for ErrMode < E1 > { # [inline] fn from_recoverable_error (token_start : & < I as Stream > :: Checkpoint , err_start : & < I as Stream > :: Checkpoint , input : & I , e : ErrMode < E2 > ,) -> Self { e . map (| e | E1 :: from_recoverable_error (token_start , err_start , input , e)) } }
};
}
