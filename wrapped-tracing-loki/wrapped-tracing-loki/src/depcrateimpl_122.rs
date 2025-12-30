// Generated macro for impl_122 (impl)
macro_rules! Depcrateimpl_122 {
() => {
// Module: crate
// Provides: {"impl_122"}
// Dependencies: {}
impl BackgroundTaskController { # [doc = " Shut down the associated `BackgroundTask`."] pub async fn shutdown (& self) { let _ = self . sender . send (None) . await ; } }
};
}
