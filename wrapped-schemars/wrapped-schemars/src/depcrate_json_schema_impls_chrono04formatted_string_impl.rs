// Generated macro for formatted_string_impl (macro)
macro_rules! Depcrate_json_schema_impls_chrono04formatted_string_impl {
() => {
// Module: crate::json_schema_impls::chrono04
// Provides: {"formatted_string_impl"}
// Dependencies: {}
macro_rules ! formatted_string_impl { ($ ty : ident , $ format : literal) => { formatted_string_impl ! ($ ty , $ format , JsonSchema for $ ty) ; } ; ($ ty : ident , $ format : literal , $ ($ desc : tt) +) => { impl $ ($ desc) + { inline_schema ! () ; fn schema_name () -> Cow <'static , str > { stringify ! ($ ty) . into () } fn schema_id () -> Cow <'static , str > { stringify ! (chrono ::$ ty) . into () } fn json_schema (_ : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : "string" , "format" : $ format }) } } } ; }
};
}
