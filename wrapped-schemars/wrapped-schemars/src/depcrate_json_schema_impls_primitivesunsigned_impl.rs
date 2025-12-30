// Generated macro for unsigned_impl (macro)
macro_rules! Depcrate_json_schema_impls_primitivesunsigned_impl {
() => {
// Module: crate::json_schema_impls::primitives
// Provides: {"unsigned_impl"}
// Dependencies: {}
macro_rules ! unsigned_impl { ($ type : ty => $ instance_type : literal , $ format : literal) => { impl JsonSchema for $ type { inline_schema ! () ; fn schema_name () -> Cow <'static , str > { $ format . into () } fn json_schema (_ : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : $ instance_type , "format" : $ format , "minimum" : 0 }) } } } ; }
};
}
