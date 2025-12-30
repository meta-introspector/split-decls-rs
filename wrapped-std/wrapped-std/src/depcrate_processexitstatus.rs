// Generated macro for ExitStatus (struct)
macro_rules! Depcrate_processExitStatus {
() => {
// Module: crate::process
// Provides: {"ExitStatus"}
// Dependencies: {}
# [doc = " Describes the result of a process after it has terminated."] # [doc = ""] # [doc = " This `struct` is used to represent the exit status or other termination of a child process."] # [doc = " Child processes are created via the [`Command`] struct and their exit"] # [doc = " status is exposed through the [`status`] method, or the [`wait`] method"] # [doc = " of a [`Child`] process."] # [doc = ""] # [doc = " An `ExitStatus` represents every possible disposition of a process.  On Unix this"] # [doc = " is the **wait status**.  It is *not* simply an *exit status* (a value passed to `exit`)."] # [doc = ""] # [doc = " For proper error reporting of failed processes, print the value of `ExitStatus` or"] # [doc = " `ExitStatusError` using their implementations of [`Display`](crate::fmt::Display)."] # [doc = ""] # [doc = " # Differences from `ExitCode`"] # [doc = ""] # [doc = " [`ExitCode`] is intended for terminating the currently running process, via"] # [doc = " the `Termination` trait, in contrast to `ExitStatus`, which represents the"] # [doc = " termination of a child process. These APIs are separate due to platform"] # [doc = " compatibility differences and their expected usage; it is not generally"] # [doc = " possible to exactly reproduce an `ExitStatus` from a child for the current"] # [doc = " process after the fact."] # [doc = ""] # [doc = " [`status`]: Command::status"] # [doc = " [`wait`]: Child::wait"] # [derive (PartialEq , Eq , Clone , Copy , Debug)] # [stable (feature = "process" , since = "1.0.0")] pub struct ExitStatus (imp :: ExitStatus) ;
};
}
