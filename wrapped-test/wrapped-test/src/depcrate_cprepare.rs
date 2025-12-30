// Generated macro for prepare (function)
macro_rules! Depcrate_cprepare {
() => {
// Module: crate::c
// Provides: {"prepare"}
// Dependencies: {}
fn prepare (runner : & mut Runner < '_ > , compiler : PathBuf) -> Result < () > { let cwd = env :: current_dir () ? ; let dir = cwd . join (& runner . opts . artifacts) . join ("c") ; super :: write_if_different (& dir . join ("test.c") , "int main() { return 0; }") ? ; println ! ("Testing if `{}` works..." , compiler . display ()) ; runner . run_command (Command :: new (& compiler) . current_dir (& dir) . arg ("test.c")) . inspect_err (| _ | { eprintln ! ("Error: failed to find `{}`. Hint: pass `--wasi-sdk-path` or set `WASI_SDK_PATH`" , compiler . display ()) ; }) ? ; Ok (()) }
};
}
