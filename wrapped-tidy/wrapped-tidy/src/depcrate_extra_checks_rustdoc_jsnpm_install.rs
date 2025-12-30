// Generated macro for npm_install (function)
macro_rules! Depcrate_extra_checks_rustdoc_jsnpm_install {
() => {
// Module: crate::extra_checks::rustdoc_js
// Provides: {"npm_install"}
// Dependencies: {}
# [doc = " install all js dependencies from package.json."] pub (super) fn npm_install (root_path : & Path , outdir : & Path , npm : & Path) -> Result < () , super :: Error > { npm :: install (root_path , outdir , npm) ? ; Ok (()) }
};
}
