// Generated macro for spellcheck_runner (function)
macro_rules! Depcrate_extra_checksspellcheck_runner {
() => {
// Module: crate::extra_checks
// Provides: {"spellcheck_runner"}
// Dependencies: {}
# [doc = " Ensure that spellchecker is installed then run it at the given path"] fn spellcheck_runner (src_root : & Path , outdir : & Path , cargo : & Path , args : & [& str] ,) -> Result < () , Error > { let bin_path = crate :: ensure_version_or_cargo_install (outdir , cargo , "typos-cli" , "typos" , "1.34.0") ? ; match Command :: new (bin_path) . current_dir (src_root) . args (args) . status () { Ok (status) => { if status . success () { Ok (()) } else { Err (Error :: FailedCheck ("typos")) } } Err (err) => Err (Error :: Generic (format ! ("failed to run typos tool: {err:?}"))) , } }
};
}
