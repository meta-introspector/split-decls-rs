// Generated macro for set_handle (function)
macro_rules! Depcrateset_handle {
() => {
// Module: crate
// Provides: {"set_handle"}
// Dependencies: {}
fn set_handle (handle : SERVICE_STATUS_HANDLE) { let mut writer = STATE . write () . unwrap () ; writer . handle = handle ; }
};
}
