// Generated macro for impl_540 (impl)
macro_rules! Depcrate_streamimpl_540 {
() => {
// Module: crate::stream
// Provides: {"impl_540"}
// Dependencies: {}
# [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] impl < E > Recover < E > for & str { # [inline (always)] fn record_err (& mut self , _token_start : & Self :: Checkpoint , _err_start : & Self :: Checkpoint , err : E ,) -> Result < () , E > { Err (err) } # [doc = " Report whether the [`Stream`] can save off errors for recovery"] # [inline (always)] fn is_recovery_supported () -> bool { false } }
};
}
