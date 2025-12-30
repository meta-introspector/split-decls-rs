// Generated macro for write_stderr_delimiter (function)
macro_rules! Depcrate_formatterswrite_stderr_delimiter {
() => {
// Module: crate::formatters
// Provides: {"write_stderr_delimiter"}
// Dependencies: {}
pub (crate) fn write_stderr_delimiter (test_output : & mut Vec < u8 > , test_name : & TestName) { match test_output . last () { Some (b'\n') => () , Some (_) => test_output . push (b'\n') , None => () , } writeln ! (test_output , "---- {test_name} stderr ----") . unwrap () ; }
};
}
