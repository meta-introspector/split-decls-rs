// Generated macro for format_rust_file (function)
macro_rules! Depcrate_rustfmt_utilsformat_rust_file {
() => {
// Module: crate::rustfmt_utils
// Provides: {"format_rust_file"}
// Dependencies: {}
pub fn format_rust_file (content : & str , path : & Path) -> Result < String > { if std :: env :: var ("SPLIT_DECLS_DEBUG") . is_ok () { eprintln ! ("DEBUG: Skipping formatting for {}" , path . display ()) ; } Ok (content . to_string ()) }
};
}
