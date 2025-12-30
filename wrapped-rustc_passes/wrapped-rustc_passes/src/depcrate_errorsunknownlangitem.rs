// Generated macro for UnknownLangItem (struct)
macro_rules! Depcrate_errorsUnknownLangItem {
() => {
// Module: crate::errors
// Provides: {"UnknownLangItem"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (passes_unknown_lang_item , code = E0522)] pub (crate) struct UnknownLangItem { # [primary_span] # [label] pub span : Span , pub name : Symbol , }
};
}
