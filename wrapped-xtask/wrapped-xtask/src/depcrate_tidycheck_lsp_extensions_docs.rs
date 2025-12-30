// Generated macro for check_lsp_extensions_docs (function)
macro_rules! Depcrate_tidycheck_lsp_extensions_docs {
() => {
// Module: crate::tidy
// Provides: {"check_lsp_extensions_docs"}
// Dependencies: {}
fn check_lsp_extensions_docs (sh : & Shell) { let expected_hash = { let lsp_ext_rs = sh . read_file (project_root () . join ("crates/rust-analyzer/src/lsp/ext.rs")) . unwrap () ; stable_hash (lsp_ext_rs . as_str ()) } ; let actual_hash = { let lsp_extensions_md = sh . read_file (project_root () . join ("docs/book/src/contributing/lsp-extensions.md")) . unwrap () ; let text = lsp_extensions_md . lines () . find_map (| line | line . strip_prefix ("lsp/ext.rs hash:")) . unwrap () . trim () ; u64 :: from_str_radix (text , 16) . unwrap () } ; if actual_hash != expected_hash { panic ! ("
lsp/ext.rs was changed without touching lsp-extensions.md.

Expected hash: {expected_hash:x}
Actual hash:   {actual_hash:x}

Please adjust docs/book/src/contributing/lsp-extensions.md.
") } }
};
}
