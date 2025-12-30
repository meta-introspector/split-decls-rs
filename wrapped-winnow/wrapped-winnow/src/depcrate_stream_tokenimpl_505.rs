// Generated macro for impl_505 (impl)
macro_rules! Depcrate_stream_tokenimpl_505 {
() => {
// Module: crate::stream::token
// Provides: {"impl_505"}
// Dependencies: {}
# [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] impl < T , E > Recover < E > for TokenSlice < '_ , T > where T : core :: fmt :: Debug + Clone , { # [inline (always)] fn record_err (& mut self , _token_start : & Self :: Checkpoint , _err_start : & Self :: Checkpoint , err : E ,) -> Result < () , E > { Err (err) } # [doc = " Report whether the [`Stream`] can save off errors for recovery"] # [inline (always)] fn is_recovery_supported () -> bool { false } }
};
}
