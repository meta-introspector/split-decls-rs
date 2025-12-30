// Generated macro for typecheck (function)
macro_rules! Depcrate_extra_checks_rustdoc_jstypecheck {
() => {
// Module: crate::extra_checks::rustdoc_js
// Provides: {"typecheck"}
// Dependencies: {}
pub (super) fn typecheck (outdir : & Path , librustdoc_path : & Path) -> Result < () , super :: Error > { let mut child = spawn_cmd (Command :: new (node_module_bin (outdir , "tsc")) . arg ("-p") . arg (librustdoc_path . join ("html/static/js/tsconfig.json")) ,) ? ; match child . wait () { Ok (exit_status) => { if exit_status . success () { return Ok (()) ; } Err (super :: Error :: FailedCheck ("tsc")) } Err (error) => Err (super :: Error :: Generic (format ! ("tsc command failed: {error:?}"))) , } }
};
}
