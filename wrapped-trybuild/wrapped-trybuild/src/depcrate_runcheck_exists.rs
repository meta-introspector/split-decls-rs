// Generated macro for check_exists (function)
macro_rules! Depcrate_runcheck_exists {
() => {
// Module: crate::run
// Provides: {"check_exists"}
// Dependencies: {}
fn check_exists (path : & Path) -> Result < () > { if path . exists () { return Ok (()) ; } match File :: open (path) { Ok (_) => Ok (()) , Err (err) => Err (Error :: Open (path . to_owned () , err)) , } }
};
}
