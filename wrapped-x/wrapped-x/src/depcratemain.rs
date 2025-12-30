// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { if env :: args () . nth (1) . is_some_and (| s | s == "--wrapper-version") { let version = env ! ("CARGO_PKG_VERSION") ; println ! ("{version}") ; return ; } let current = match env :: current_dir () { Ok (dir) => dir , Err (err) => { eprintln ! ("Failed to get current directory: {err}") ; process :: exit (1) ; } } ; for dir in current . ancestors () { let candidate = dir . join ("x.py") ; if candidate . exists () { let shell_script_candidate = dir . join ("x") ; let mut cmd : Command ; if shell_script_candidate . exists () { cmd = x_command (dir) ; cmd . args (env :: args () . skip (1)) . current_dir (dir) ; } else { cmd = Command :: new (python ()) ; cmd . arg (& candidate) . args (env :: args () . skip (1)) . current_dir (dir) ; } let result = exec_or_status (& mut cmd) ; handle_result (result , cmd) ; } } eprintln ! ("x.py not found. Please run inside of a checkout of `https://github.com/rust-lang/rust`.") ; process :: exit (1) ; }
};
}
