// Generated macro for macro_73 (macro)
macro_rules! Depcrate_sync_cancellation_tokenmacro_73 {
() => {
// Module: crate::sync::cancellation_token
// Provides: {"macro_73"}
// Dependencies: {}
pin_project ! { # [doc = " A Future that is resolved once the corresponding [`CancellationToken`]"] # [doc = " is cancelled or a given Future gets resolved. It is biased towards the"] # [doc = " Future completion."] # [must_use = "futures do nothing unless polled"] pub (crate) struct RunUntilCancelledFutureOwned < F : Future > { # [pin] cancellation : WaitForCancellationFutureOwned , # [pin] future : F , } }
};
}
