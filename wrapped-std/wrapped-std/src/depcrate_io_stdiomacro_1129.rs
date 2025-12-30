// Generated macro for macro_1129 (macro)
macro_rules! Depcrate_io_stdiomacro_1129 {
() => {
// Module: crate::io::stdio
// Provides: {"macro_1129"}
// Dependencies: {}
thread_local ! { # [doc = " Used by the test crate to capture the output of the print macros and panics."] static OUTPUT_CAPTURE : Cell < Option < LocalStream >> = const { Cell :: new (None) } }
};
}
