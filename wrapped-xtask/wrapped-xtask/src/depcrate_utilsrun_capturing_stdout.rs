// Generated macro for run_capturing_stdout (function)
macro_rules! Depcrate_utilsrun_capturing_stdout {
() => {
// Module: crate::utils
// Provides: {"run_capturing_stdout"}
// Dependencies: {}
# [doc = " Execute the [`Command`]. If success return `stdout`, if failure print to `stderr`"] pub fn run_capturing_stdout (cmd : & mut Command) -> anyhow :: Result < String > { let output = cmd . output () ? ; match output . status . success () { true => Ok (str :: from_utf8 (& output . stdout) ? . to_string ()) , false => { eprintln ! ("{}" , str :: from_utf8 (& output . stderr) ?. dimmed ()) ; Err (anyhow ! ("")) } } }
};
}
