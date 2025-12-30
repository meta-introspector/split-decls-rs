// Generated macro for attempt_print_to_stderr (function)
macro_rules! Depcrate_io_stdioattempt_print_to_stderr {
() => {
// Module: crate::io::stdio
// Provides: {"attempt_print_to_stderr"}
// Dependencies: {}
# [doc = " Used by impl Termination for Result to print error after `main` or a test"] # [doc = " has returned. Should avoid panicking, although we can't help it if one of"] # [doc = " the Display impls inside args decides to."] pub (crate) fn attempt_print_to_stderr (args : fmt :: Arguments < '_ >) { if print_to_buffer_if_capture_used (args) { return ; } let _ = stderr () . write_fmt (args) ; }
};
}
