// Generated macro for impl_212 (impl)
macro_rules! Depcrate_stream_bstrimpl_212 {
() => {
// Module: crate::stream::bstr
// Provides: {"impl_212"}
// Dependencies: {}
# [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] impl < E > Recover < E > for & BStr { # [inline (always)] fn record_err (& mut self , _token_start : & Self :: Checkpoint , _err_start : & Self :: Checkpoint , err : E ,) -> Result < () , E > { Err (err) } # [doc = " Report whether the [`Stream`] can save off errors for recovery"] # [inline (always)] fn is_recovery_supported () -> bool { false } }
};
}
