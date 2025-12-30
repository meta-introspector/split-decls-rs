// Generated macro for service_thread (function)
macro_rules! Depcrateservice_thread {
() => {
// Module: crate
// Provides: {"service_thread"}
// Dependencies: {}
fn service_thread () { for i in 0 .. 10 { log (& format ! ("...{i}\n")) ; std :: thread :: sleep (std :: time :: Duration :: from_millis (1000)) ; if stop_request () { break ; } } }
};
}
