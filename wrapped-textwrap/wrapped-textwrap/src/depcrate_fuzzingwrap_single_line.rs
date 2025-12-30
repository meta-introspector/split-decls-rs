// Generated macro for wrap_single_line (function)
macro_rules! Depcrate_fuzzingwrap_single_line {
() => {
// Module: crate::fuzzing
// Provides: {"wrap_single_line"}
// Dependencies: {}
# [doc = " Exposed for fuzzing so we can check the slow path is correct."] pub fn wrap_single_line < 'a > (line : & 'a str , options : & Options < '_ > , lines : & mut Vec < Cow < 'a , str > >) { crate :: wrap :: wrap_single_line (line , options , lines) ; }
};
}
