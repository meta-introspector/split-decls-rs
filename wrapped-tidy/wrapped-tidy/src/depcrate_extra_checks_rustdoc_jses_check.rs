// Generated macro for es_check (function)
macro_rules! Depcrate_extra_checks_rustdoc_jses_check {
() => {
// Module: crate::extra_checks::rustdoc_js
// Provides: {"es_check"}
// Dependencies: {}
pub (super) fn es_check (outdir : & Path , librustdoc_path : & Path) -> Result < () , super :: Error > { let files_to_check = rustdoc_js_files (librustdoc_path) ; let mut cmd = Command :: new (node_module_bin (outdir , "es-check")) ; cmd . arg ("es2019") ; for f in files_to_check { cmd . arg (f) ; } match spawn_cmd (& mut cmd) ? . wait () { Ok (exit_status) => { if exit_status . success () { return Ok (()) ; } Err (super :: Error :: FailedCheck ("es-check")) } Err (error) => Err (super :: Error :: Generic (format ! ("es-check command failed: {error:?}"))) , } }
};
}
