// Generated macro for run_command (function)
macro_rules! Depcraterun_command {
() => {
// Module: crate
// Provides: {"run_command"}
// Dependencies: {}
# [doc = " Helper function to execute a process during tests and print informative"] # [doc = " information if it fails."] pub fn run_command (cmd : & mut Command) { let command = format ! ("{cmd:?}") ; let output = cmd . output () . unwrap_or_else (| e | panic ! ("failed to run executable: {e}; command was `{command}`")) ; if output . status . success () { return ; } panic ! ("
command: {cmd:?}
status: {status}

stdout ---
{stdout}

stderr ---
{stderr}" , status = output . status , stdout = String :: from_utf8_lossy (& output . stdout) . replace ("\n" , "\n\t") , stderr = String :: from_utf8_lossy (& output . stderr) . replace ("\n" , "\n\t") ,) ; }
};
}
