// Generated macro for impl_130 (impl)
macro_rules! Depcrate_tidyimpl_130 {
() => {
// Module: crate::tidy
// Provides: {"impl_130"}
// Dependencies: {}
impl Tidy { pub (crate) fn run (& self , sh : & Shell) -> anyhow :: Result < () > { check_lsp_extensions_docs (sh) ; files_are_tidy (sh) ; check_licenses (sh) ; Ok (()) } }
};
}
