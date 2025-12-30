// Generated macro for test_httpd (function)
macro_rules! Depcrate_ci_qemutest_httpd {
() => {
// Module: crate::ci::qemu
// Provides: {"test_httpd"}
// Dependencies: {}
fn test_httpd (guest_ip : IpAddr) -> Result < () > { thread :: sleep (Duration :: from_secs (10)) ; let url = format ! ("http://{guest_ip}:9975") ; eprintln ! ("[CI] GET {url}") ; let body = ureq :: get (url) . config () . timeout_global (Some (Duration :: from_secs (3))) . build () . call () ? . into_body () . read_to_string () ? ; eprintln ! ("[CI] {body}") ; assert_eq ! (body . lines () . next () , Some ("Hello from Hermit! 🦀")) ; Ok (()) }
};
}
