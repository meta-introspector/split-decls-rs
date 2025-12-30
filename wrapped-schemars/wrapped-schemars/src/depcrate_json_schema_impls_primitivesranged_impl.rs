// Generated macro for ranged_impl (macro)
macro_rules! Depcrate_json_schema_impls_primitivesranged_impl {
() => {
// Module: crate::json_schema_impls::primitives
// Provides: {"ranged_impl"}
// Dependencies: {}
macro_rules ! ranged_impl { ($ type : ty => $ instance_type : literal , $ format : literal) => { impl JsonSchema for $ type { inline_schema ! () ; fn schema_name () -> Cow <'static , str > { $ format . into () } fn json_schema (_ : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : $ instance_type , "format" : $ format , "minimum" : <$ type >:: MIN , "maximum" : <$ type >:: MAX }) } } } ; }
};
}
