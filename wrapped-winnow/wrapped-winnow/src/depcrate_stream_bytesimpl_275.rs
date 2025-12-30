// Generated macro for impl_275 (impl)
macro_rules! Depcrate_stream_bytesimpl_275 {
() => {
// Module: crate::stream::bytes
// Provides: {"impl_275"}
// Dependencies: {}
# [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] impl < E > Recover < E > for & Bytes { # [inline (always)] fn record_err (& mut self , _token_start : & Self :: Checkpoint , _err_start : & Self :: Checkpoint , err : E ,) -> Result < () , E > { Err (err) } # [doc = " Report whether the [`Stream`] can save off errors for recovery"] # [inline (always)] fn is_recovery_supported () -> bool { false } }
};
}
