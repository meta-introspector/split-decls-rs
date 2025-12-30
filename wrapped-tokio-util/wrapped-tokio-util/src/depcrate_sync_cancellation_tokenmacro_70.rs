// Generated macro for macro_70 (macro)
macro_rules! Depcrate_sync_cancellation_tokenmacro_70 {
() => {
// Module: crate::sync::cancellation_token
// Provides: {"macro_70"}
// Dependencies: {}
pin_project ! { # [doc = " A Future that is resolved once the corresponding [`CancellationToken`]"] # [doc = " is cancelled or a given Future gets resolved. It is biased towards the"] # [doc = " Future completion."] # [must_use = "futures do nothing unless polled"] pub (crate) struct RunUntilCancelledFuture <'a , F : Future > { # [pin] cancellation : WaitForCancellationFuture <'a >, # [pin] future : F , } }
};
}
