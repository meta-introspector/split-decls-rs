// Generated macro for callback (function)
macro_rules! Depcratecallback {
() => {
// Module: crate
// Provides: {"callback"}
// Dependencies: {}
extern "system" fn callback (_ : PTP_CALLBACK_INSTANCE , _ : * mut std :: ffi :: c_void , _ : PTP_WORK) { let mut counter = COUNTER . write () . unwrap () ; * counter += 1 ; }
};
}
