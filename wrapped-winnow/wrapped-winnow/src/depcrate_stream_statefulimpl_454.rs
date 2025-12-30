// Generated macro for impl_454 (impl)
macro_rules! Depcrate_stream_statefulimpl_454 {
() => {
// Module: crate::stream::stateful
// Provides: {"impl_454"}
// Dependencies: {}
# [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] impl < I , E , S > Recover < E > for Stateful < I , S > where I : Recover < E > , I : Stream , S : Clone + core :: fmt :: Debug , { # [inline (always)] fn record_err (& mut self , _token_start : & Self :: Checkpoint , _err_start : & Self :: Checkpoint , err : E ,) -> Result < () , E > { Err (err) } # [doc = " Report whether the [`Stream`] can save off errors for recovery"] # [inline (always)] fn is_recovery_supported () -> bool { false } }
};
}
