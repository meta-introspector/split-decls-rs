// Generated macro for nonzero_signed_impl (macro)
macro_rules! Depcrate_json_schema_impls_nonzerononzero_signed_impl {
() => {
// Module: crate::json_schema_impls::nonzero
// Provides: {"nonzero_signed_impl"}
// Dependencies: {}
macro_rules ! nonzero_signed_impl { ($ type : ty => $ primitive : ty) => { impl JsonSchema for $ type { inline_schema ! () ; fn schema_name () -> Cow <'static , str > { stringify ! ($ type) . into () } fn schema_id () -> Cow <'static , str > { stringify ! (std :: num ::$ type) . into () } fn json_schema (generator : & mut SchemaGenerator) -> Schema { let mut schema = <$ primitive >:: json_schema (generator) ; schema . insert ("not" . to_owned () , serde_json :: json ! ({ "const" : 0 })) ; schema } } } ; }
};
}
