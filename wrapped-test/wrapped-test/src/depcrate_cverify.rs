// Generated macro for verify (function)
macro_rules! Depcrate_cverify {
() => {
// Module: crate::c
// Provides: {"verify"}
// Dependencies: {}
fn verify (runner : & Runner < '_ > , verify : & Verify < '_ > , compiler : PathBuf) -> Result < () > { let mut cmd = Command :: new (compiler) ; cmd . arg (verify . bindings_dir . join (format ! ("{}.c" , verify . world . to_snake_case ())) ,) . arg ("-I") . arg (& verify . bindings_dir) . arg ("-Wall") . arg ("-Wextra") . arg ("-Werror") . arg ("-Wc++-compat") . arg ("-Wno-unused-parameter") . arg ("-c") . arg ("-o") . arg (verify . artifacts_dir . join ("tmp.o")) ; runner . run_command (& mut cmd) }
};
}
