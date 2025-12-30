// Generated macro for macro_58 (macro)
macro_rules! Depcrate_sync_cancellation_tokenmacro_58 {
() => {
// Module: crate::sync::cancellation_token
// Provides: {"macro_58"}
// Dependencies: {}
pin_project ! { # [doc = " A Future that is resolved once the corresponding [`CancellationToken`]"] # [doc = " is cancelled."] # [must_use = "futures do nothing unless polled"] pub struct WaitForCancellationFuture <'a > { cancellation_token : &'a CancellationToken , # [pin] future : tokio :: sync :: futures :: Notified <'a >, } }
};
}
