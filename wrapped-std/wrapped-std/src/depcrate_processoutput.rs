// Generated macro for Output (struct)
macro_rules! Depcrate_processOutput {
() => {
// Module: crate::process
// Provides: {"Output"}
// Dependencies: {}
# [doc = " The output of a finished process."] # [doc = ""] # [doc = " This is returned in a Result by either the [`output`] method of a"] # [doc = " [`Command`], or the [`wait_with_output`] method of a [`Child`]"] # [doc = " process."] # [doc = ""] # [doc = " [`output`]: Command::output"] # [doc = " [`wait_with_output`]: Child::wait_with_output"] # [derive (PartialEq , Eq , Clone)] # [stable (feature = "process" , since = "1.0.0")] pub struct Output { # [doc = " The status (exit code) of the process."] # [stable (feature = "process" , since = "1.0.0")] pub status : ExitStatus , # [doc = " The data that the process wrote to stdout."] # [stable (feature = "process" , since = "1.0.0")] pub stdout : Vec < u8 > , # [doc = " The data that the process wrote to stderr."] # [stable (feature = "process" , since = "1.0.0")] pub stderr : Vec < u8 > , }
};
}
