// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { println ! ("Hello from the self-application example!") ; # [cfg (self_application_build)] println ! ("This code is enabled because the 'self_application_build' cfg flag was set by our own build.rs script!") ; # [cfg (not (self_application_build))] println ! ("This code should not be enabled.") ; }
};
}
