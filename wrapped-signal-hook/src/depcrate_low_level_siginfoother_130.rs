// Generated macro for other_130 (other)
macro_rules! Depcrate_low_level_siginfoother_130 {
() => {
// Module: crate::low_level::siginfo
// Provides: {"other_130"}
// Dependencies: {}
extern "C" { fn sighook_signal_cause (info : & siginfo_t) -> ICause ; fn sighook_signal_pid (info : & siginfo_t) -> pid_t ; fn sighook_signal_uid (info : & siginfo_t) -> uid_t ; }
};
}
