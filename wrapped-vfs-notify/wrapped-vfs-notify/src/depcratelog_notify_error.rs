// Generated macro for log_notify_error (function)
macro_rules! Depcratelog_notify_error {
() => {
// Module: crate
// Provides: {"log_notify_error"}
// Dependencies: {}
fn log_notify_error < T > (res : notify :: Result < T >) -> Option < T > { res . map_err (| err | tracing :: warn ! ("notify error: {}" , err)) . ok () }
};
}
