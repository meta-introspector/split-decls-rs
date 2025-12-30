// Generated macro for run_input_output (function)
macro_rules! Depcrate_utilrun_input_output {
() => {
// Module: crate::util
// Provides: {"run_input_output"}
// Dependencies: {}
# [doc = " Run `command`, writing `input` to its stdin if it is `Some` and return the exit status and output."] # [doc = ""] # [doc = " If the command returns a non-successful exit status, an error of `SccacheError::ProcessError`"] # [doc = " will be returned containing the process output."] pub async fn run_input_output < C > (mut command : C , input : Option < Vec < u8 > >) -> Result < process :: Output > where C : RunCommand , { let child = command . stdin (if input . is_some () { Stdio :: piped () } else { Stdio :: inherit () }) . stdout (Stdio :: piped ()) . stderr (Stdio :: piped ()) . spawn () . await ? ; wait_with_input_output (child , input) . await . and_then (| output | { if output . status . success () { Ok (output) } else { Err (ProcessError (output) . into ()) } }) }
};
}
