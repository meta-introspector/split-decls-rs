// Generated macro for UnknownExternLangItem (struct)
macro_rules! Depcrate_errorsUnknownExternLangItem {
() => {
// Module: crate::errors
// Provides: {"UnknownExternLangItem"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (passes_unknown_external_lang_item , code = E0264)] pub (crate) struct UnknownExternLangItem { # [primary_span] pub span : Span , pub lang_item : Symbol , }
};
}
