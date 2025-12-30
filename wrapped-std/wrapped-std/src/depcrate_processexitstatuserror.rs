// Generated macro for ExitStatusError (struct)
macro_rules! Depcrate_processExitStatusError {
() => {
// Module: crate::process
// Provides: {"ExitStatusError"}
// Dependencies: {}
# [doc = " Describes the result of a process after it has failed"] # [doc = ""] # [doc = " Produced by the [`.exit_ok`](ExitStatus::exit_ok) method on [`ExitStatus`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #![feature(exit_status_error)]"] # [doc = " # if cfg!(all(unix, not(target_os = \"android\"))) {"] # [doc = " use std::process::{Command, ExitStatusError};"] # [doc = ""] # [doc = " fn run(cmd: &str) -> Result<(), ExitStatusError> {"] # [doc = "     Command::new(cmd).status().unwrap().exit_ok()?;"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = ""] # [doc = " run(\"true\").unwrap();"] # [doc = " run(\"false\").unwrap_err();"] # [doc = " # } // cfg!(unix)"] # [doc = " ```"] # [derive (PartialEq , Eq , Clone , Copy , Debug)] # [unstable (feature = "exit_status_error" , issue = "84908")] pub struct ExitStatusError (imp :: ExitStatusError) ;
};
}
