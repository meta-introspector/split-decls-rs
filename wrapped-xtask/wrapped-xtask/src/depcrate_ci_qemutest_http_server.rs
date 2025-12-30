// Generated macro for test_http_server (function)
macro_rules! Depcrate_ci_qemutest_http_server {
() => {
// Module: crate::ci::qemu
// Provides: {"test_http_server"}
// Dependencies: {}
fn test_http_server (guest_ip : IpAddr) -> Result < () > { thread :: sleep (Duration :: from_secs (10)) ; let url = format ! ("http://{guest_ip}:9975") ; eprintln ! ("[CI] GET {url}") ; let body = ureq :: get (url) . config () . timeout_global (Some (Duration :: from_secs (3))) . build () . call () ? . into_body () . read_to_string () ? ; eprintln ! ("[CI] body = {body:?}") ; assert_eq ! (body , "Hello, world!\n") ; Ok (()) }
};
}
