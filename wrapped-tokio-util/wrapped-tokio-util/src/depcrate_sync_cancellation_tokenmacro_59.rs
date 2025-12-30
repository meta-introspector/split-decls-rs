// Generated macro for macro_59 (macro)
macro_rules! Depcrate_sync_cancellation_tokenmacro_59 {
() => {
// Module: crate::sync::cancellation_token
// Provides: {"macro_59"}
// Dependencies: {}
pin_project ! { # [doc = " A Future that is resolved once the corresponding [`CancellationToken`]"] # [doc = " is cancelled."] # [doc = ""] # [doc = " This is the counterpart to [`WaitForCancellationFuture`] that takes"] # [doc = " [`CancellationToken`] by value instead of using a reference."] # [must_use = "futures do nothing unless polled"] pub struct WaitForCancellationFutureOwned { # [pin] future : MaybeDangling < tokio :: sync :: futures :: Notified <'static >>, cancellation_token : CancellationToken , } }
};
}
