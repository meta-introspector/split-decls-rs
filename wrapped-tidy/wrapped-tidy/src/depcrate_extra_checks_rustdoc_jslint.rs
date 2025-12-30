// Generated macro for lint (function)
macro_rules! Depcrate_extra_checks_rustdoc_jslint {
() => {
// Module: crate::extra_checks::rustdoc_js
// Provides: {"lint"}
// Dependencies: {}
pub (super) fn lint (outdir : & Path , librustdoc_path : & Path , tools_path : & Path , bless : bool ,) -> Result < () , super :: Error > { let files_to_check = rustdoc_js_files (librustdoc_path) ; println ! ("Running eslint on rustdoc JS files") ; run_eslint (outdir , & files_to_check , librustdoc_path . join ("html/static") , bless) ? ; run_eslint (outdir , & [tools_path . join ("rustdoc-js/tester.js")] , tools_path . join ("rustdoc-js") , bless ,) ? ; Ok (()) }
};
}
