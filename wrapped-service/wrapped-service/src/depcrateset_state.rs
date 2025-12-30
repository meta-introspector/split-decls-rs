// Generated macro for set_state (function)
macro_rules! Depcrateset_state {
() => {
// Module: crate
// Provides: {"set_state"}
// Dependencies: {}
fn set_state (state : SERVICE_STATUS_CURRENT_STATE) { let mut writer = STATE . write () . unwrap () ; writer . status . dwCurrentState = state ; let handle = writer . handle ; let status = writer . status ; drop (writer) ; unsafe { SetServiceStatus (handle , & status) ; } }
};
}
