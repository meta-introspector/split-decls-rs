// Generated macro for impl_94 (impl)
macro_rules! Depcrate_runnerimpl_94 {
() => {
// Module: crate::runner
// Provides: {"impl_94"}
// Dependencies: {}
impl TestRunner { pub fn new (runner : & OsString) -> Result < TestRunner > { let original_err = match Command :: new (runner) . arg ("--version") . output () { Ok (_) => { return Ok (TestRunner { path : runner . clone () , args : Vec :: new () , }) } Err (e) => e , } ; let runner_and_args = runner . to_str () . context ("--runner argument is not utf-8") ? ; let mut delimited = runner_and_args . split_whitespace () ; let command = delimited . next () . unwrap () ; if Command :: new (command) . arg ("--version") . output () . is_ok () { return Ok (TestRunner { path : command . into () , args : delimited . map (| s | s . to_string ()) . collect () , }) ; } Err (original_err) . context (format ! ("runner `{runner_and_args}` failed to spawn")) } # [doc = " Returns a `Command` which can be used to execute this test runner."] pub fn command (& self) -> Command { let mut ret = Command :: new (& self . path) ; for arg in self . args . iter () { ret . arg (arg) ; } ret } }
};
}
