// Generated macro for cmd_with (function)
macro_rules! Depcratecmd_with {
() => {
// Module: crate
// Provides: {"cmd_with"}
// Dependencies: {}
fn cmd_with < F > (program : & str , args : & [& str] , f : F) -> Result < () , DynError > where F : FnOnce (& mut Command) , { println ! ("Running '{} {}'" , program , args . join (" ")) ; let mut command = Command :: new (program) ; command . current_dir (project_root ()) . args (args) ; f (& mut command) ; let status = command . status () ? ; if ! status . success () { Err (format ! ("'{} {}' failed" , program , args . join (" "))) ? ; } Ok (()) }
};
}
