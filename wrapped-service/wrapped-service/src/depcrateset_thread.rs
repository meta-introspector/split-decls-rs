// Generated macro for set_thread (function)
macro_rules! Depcrateset_thread {
() => {
// Module: crate
// Provides: {"set_thread"}
// Dependencies: {}
fn set_thread () { let thread = std :: thread :: spawn (| | { service_thread () ; if state () == SERVICE_RUNNING { set_state (SERVICE_STOPPED) ; log ("service stopped\n") ; } }) ; let mut writer = STATE . write () . unwrap () ; debug_assert ! (writer . thread . is_none ()) ; writer . thread = Some (thread) ; }
};
}
