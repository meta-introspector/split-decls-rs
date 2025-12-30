// Generated macro for service_main (function)
macro_rules! Depcrateservice_main {
() => {
// Module: crate
// Provides: {"service_main"}
// Dependencies: {}
extern "system" fn service_main (_len : u32 , _args : * mut PWSTR) { unsafe { set_handle (RegisterServiceCtrlHandlerW (std :: ptr :: null () , Some (handler))) ; } set_state (SERVICE_START_PENDING) ; log ("service start pending\n") ; set_thread () ; set_state (SERVICE_RUNNING) ; log ("service running\n") ; }
};
}
