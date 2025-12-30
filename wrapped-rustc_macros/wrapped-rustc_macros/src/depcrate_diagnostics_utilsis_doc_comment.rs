// Generated macro for is_doc_comment (function)
macro_rules! Depcrate_diagnostics_utilsis_doc_comment {
() => {
// Module: crate::diagnostics::utils
// Provides: {"is_doc_comment"}
// Dependencies: {}
pub (super) fn is_doc_comment (attr : & Attribute) -> bool { attr . path () . segments . last () . unwrap () . ident == "doc" }
};
}
