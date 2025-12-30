// Generated macro for should_generate_arg (function)
macro_rules! Depcrate_diagnostics_utilsshould_generate_arg {
() => {
// Module: crate::diagnostics::utils
// Provides: {"should_generate_arg"}
// Dependencies: {}
# [doc = " Returns `true` if `field` should generate a `arg` call rather than any other diagnostic"] # [doc = " call (like `span_label`)."] pub (super) fn should_generate_arg (field : & Field) -> bool { field . attrs . iter () . all (| attr | is_doc_comment (attr)) }
};
}
