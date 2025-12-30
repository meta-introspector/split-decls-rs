// Generated macro for current_version (function)
macro_rules! Depcrate_current_versioncurrent_version {
() => {
// Module: crate::current_version
// Provides: {"current_version"}
// Dependencies: {}
pub (crate) fn current_version (_input : TokenStream) -> TokenStream { let env_var = "CFG_RELEASE" ; TokenStream :: from (match RustcVersion :: parse_cfg_release (env_var) { Ok (RustcVersion { major , minor , patch }) => quote ! (Self { major : # major , minor : # minor , patch : # patch }) , Err (err) => syn :: Error :: new (Span :: call_site () , format ! ("{env_var} env var: {err}")) . into_compile_error () , }) }
};
}
