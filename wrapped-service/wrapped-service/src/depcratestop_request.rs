// Generated macro for stop_request (function)
macro_rules! Depcratestop_request {
() => {
// Module: crate
// Provides: {"stop_request"}
// Dependencies: {}
fn stop_request () -> bool { matches ! (state () , SERVICE_PAUSE_PENDING | SERVICE_STOP_PENDING) }
};
}
