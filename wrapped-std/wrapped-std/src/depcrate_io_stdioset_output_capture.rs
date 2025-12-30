// Generated macro for set_output_capture (function)
macro_rules! Depcrate_io_stdioset_output_capture {
() => {
// Module: crate::io::stdio
// Provides: {"set_output_capture"}
// Dependencies: {}
# [doc = " Sets the thread-local output capture buffer and returns the old one."] # [unstable (feature = "internal_output_capture" , reason = "this function is meant for use in the test crate \
        and may disappear in the future" , issue = "none")] # [doc (hidden)] pub fn set_output_capture (sink : Option < LocalStream >) -> Option < LocalStream > { try_set_output_capture (sink) . expect ("cannot access a Thread Local Storage value \
         during or after destruction" ,) }
};
}
