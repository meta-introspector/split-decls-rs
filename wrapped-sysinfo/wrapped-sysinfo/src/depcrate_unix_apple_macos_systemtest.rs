// Generated macro for test (module)
macro_rules! Depcrate_unix_apple_macos_systemtest {
() => {
// Module: crate::unix::apple::macos::system
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [doc = " Regression test for <https://github.com/GuillaumeGomez/sysinfo/issues/956>."] # [test] fn test_getting_time_interval () { if ! crate :: IS_SUPPORTED_SYSTEM || cfg ! (feature = "apple-sandbox") { return ; } # [allow (deprecated)] let port = unsafe { libc :: mach_host_self () } ; let mut info = SystemTimeInfo :: new (port) . unwrap () ; info . get_time_interval (port) ; std :: thread :: sleep (crate :: MINIMUM_CPU_UPDATE_INTERVAL . saturating_mul (5)) ; let val = info . get_time_interval (port) ; assert_ne ! (val , crate :: MINIMUM_CPU_UPDATE_INTERVAL . as_secs_f64 () * 1_000_000_000.0) ; } }
};
}
