// Generated macro for impl_1067 (impl)
macro_rules! Depcrate_nonstandard_styleimpl_1067 {
() => {
// Module: crate::nonstandard_style
// Provides: {"impl_1067"}
// Dependencies: {}
impl NonCamelCaseTypes { fn check_case (& self , cx : & EarlyContext < '_ > , sort : & str , ident : & Ident) { let name = ident . name . as_str () ; if ! is_camel_case (name) { let cc = to_camel_case (name) ; let sub = if * name != cc { NonCamelCaseTypeSub :: Suggestion { span : ident . span , replace : cc } } else { NonCamelCaseTypeSub :: Label { span : ident . span } } ; cx . emit_span_lint (NON_CAMEL_CASE_TYPES , ident . span , NonCamelCaseType { sort , name , sub } ,) ; } } }
};
}
