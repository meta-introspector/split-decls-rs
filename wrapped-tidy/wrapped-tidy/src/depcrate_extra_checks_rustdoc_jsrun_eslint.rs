// Generated macro for run_eslint (function)
macro_rules! Depcrate_extra_checks_rustdoc_jsrun_eslint {
() => {
// Module: crate::extra_checks::rustdoc_js
// Provides: {"run_eslint"}
// Dependencies: {}
fn run_eslint (outdir : & Path , args : & [PathBuf] , config_folder : PathBuf , bless : bool ,) -> Result < () , super :: Error > { let mut cmd = Command :: new (node_module_bin (outdir , "eslint")) ; if bless { cmd . arg ("--fix") ; } cmd . arg ("-c") . arg (config_folder . join (".eslintrc.js")) . args (args) ; let mut child = spawn_cmd (& mut cmd) ? ; match child . wait () { Ok (exit_status) => { if exit_status . success () { return Ok (()) ; } Err (super :: Error :: FailedCheck ("eslint")) } Err (error) => Err (super :: Error :: Generic (format ! ("eslint command failed: {error:?}"))) , } }
};
}
