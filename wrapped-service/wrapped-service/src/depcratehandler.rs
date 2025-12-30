// Generated macro for handler (function)
macro_rules! Depcratehandler {
() => {
// Module: crate
// Provides: {"handler"}
// Dependencies: {}
extern "system" fn handler (control : u32) { match control { SERVICE_CONTROL_CONTINUE if state () == SERVICE_PAUSED => { set_state (SERVICE_CONTINUE_PENDING) ; log ("service continue pending\n") ; set_thread () ; set_state (SERVICE_RUNNING) ; log ("service running\n") ; } SERVICE_CONTROL_PAUSE if state () == SERVICE_RUNNING => { set_state (SERVICE_PAUSE_PENDING) ; log ("service pause pending\n") ; join_thread () ; set_state (SERVICE_PAUSED) ; log ("service paused\n") ; } SERVICE_CONTROL_SHUTDOWN | SERVICE_CONTROL_STOP => { if state () == SERVICE_RUNNING { set_state (SERVICE_STOP_PENDING) ; log ("service stop pending\n") ; join_thread () ; } set_state (SERVICE_STOPPED) ; log ("service stopped\n") ; } _ => { log (& format ! ("ignored control: {control}\n")) ; } } }
};
}
