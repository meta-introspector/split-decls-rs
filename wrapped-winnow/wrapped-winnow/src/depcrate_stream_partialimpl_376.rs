// Generated macro for impl_376 (impl)
macro_rules! Depcrate_stream_partialimpl_376 {
() => {
// Module: crate::stream::partial
// Provides: {"impl_376"}
// Dependencies: {}
# [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] impl < I , E > Recover < E > for Partial < I > where I : Recover < E > , I : Stream , { # [inline (always)] fn record_err (& mut self , _token_start : & Self :: Checkpoint , _err_start : & Self :: Checkpoint , err : E ,) -> Result < () , E > { Err (err) } # [doc = " Report whether the [`Stream`] can save off errors for recovery"] # [inline (always)] fn is_recovery_supported () -> bool { false } }
};
}
