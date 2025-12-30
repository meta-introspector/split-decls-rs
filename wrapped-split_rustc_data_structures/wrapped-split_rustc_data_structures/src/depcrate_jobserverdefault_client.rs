// Generated macro for default_client (function)
macro_rules! Depcrate_jobserverdefault_client {
() => {
// Module: crate::jobserver
// Provides: {"default_client"}
// Dependencies: {}
fn default_client () -> Client { let client = Client :: new (32) . expect ("failed to create jobserver") ; client . acquire_raw () . ok () ; client }
};
}
