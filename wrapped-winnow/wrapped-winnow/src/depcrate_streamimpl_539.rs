// Generated macro for impl_539 (impl)
macro_rules! Depcrate_streamimpl_539 {
() => {
// Module: crate::stream
// Provides: {"impl_539"}
// Dependencies: {}
# [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] impl < 'a , T , E > Recover < E > for & 'a [T] where & 'a [T] : Stream , { # [inline (always)] fn record_err (& mut self , _token_start : & Self :: Checkpoint , _err_start : & Self :: Checkpoint , err : E ,) -> Result < () , E > { Err (err) } # [doc = " Report whether the [`Stream`] can save off errors for recovery"] # [inline (always)] fn is_recovery_supported () -> bool { false } }
};
}
