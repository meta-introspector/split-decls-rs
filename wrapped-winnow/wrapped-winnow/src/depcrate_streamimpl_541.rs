// Generated macro for impl_541 (impl)
macro_rules! Depcrate_streamimpl_541 {
() => {
// Module: crate::stream
// Provides: {"impl_541"}
// Dependencies: {}
# [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] impl < I , E > Recover < E > for (I , usize) where I : Recover < E > , I : Stream < Token = u8 > + Clone , { # [inline (always)] fn record_err (& mut self , _token_start : & Self :: Checkpoint , _err_start : & Self :: Checkpoint , err : E ,) -> Result < () , E > { Err (err) } # [doc = " Report whether the [`Stream`] can save off errors for recovery"] # [inline (always)] fn is_recovery_supported () -> bool { false } }
};
}
