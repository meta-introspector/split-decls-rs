// Generated macro for cmd (function)
macro_rules! Depcratecmd {
() => {
// Module: crate
// Provides: {"cmd"}
// Dependencies: {}
fn cmd (cmd : & str , args : & [& str]) -> Result < () , DynError > { cmd_with (cmd , args , | _ | ()) }
};
}
