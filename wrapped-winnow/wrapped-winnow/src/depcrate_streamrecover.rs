// Generated macro for Recover (trait)
macro_rules! Depcrate_streamRecover {
() => {
// Module: crate::stream
// Provides: {"Recover"}
// Dependencies: {}
# [doc = " Capture top-level errors in the middle of parsing so parsing can resume"] # [doc = ""] # [doc = " See [`Recoverable`] for adding error recovery tracking to your [`Stream`]"] # [cfg (feature = "unstable-recover")] # [cfg (feature = "std")] pub trait Recover < E > : Stream { # [doc = " Capture a top-level error"] # [doc = ""] # [doc = " May return `Err(err)` if recovery is not possible (e.g. if [`Recover::is_recovery_supported`]"] # [doc = " returns `false`)."] fn record_err (& mut self , token_start : & Self :: Checkpoint , err_start : & Self :: Checkpoint , err : E ,) -> Result < () , E > ; # [doc = " Report whether the [`Stream`] can save off errors for recovery"] fn is_recovery_supported () -> bool ; }
};
}
