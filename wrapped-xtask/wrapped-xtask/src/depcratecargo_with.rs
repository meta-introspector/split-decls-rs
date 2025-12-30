// Generated macro for cargo_with (function)
macro_rules! Depcratecargo_with {
() => {
// Module: crate
// Provides: {"cargo_with"}
// Dependencies: {}
fn cargo_with (args : & [& str] , f : impl FnOnce (& mut Command)) -> Result < () , DynError > { let cargo = env :: var ("CARGO") . unwrap_or_else (| _ | "cargo" . to_string ()) ; cmd_with (& cargo , args , f) }
};
}
