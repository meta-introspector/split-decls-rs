// Generated macro for simple_impl (macro)
macro_rules! Depcrate_json_schema_impls_primitivessimple_impl {
() => {
// Module: crate::json_schema_impls::primitives
// Provides: {"simple_impl"}
// Dependencies: {}
macro_rules ! simple_impl { ($ type : ty => $ instance_type : literal) => { impl JsonSchema for $ type { inline_schema ! () ; fn schema_name () -> Cow <'static , str > { $ instance_type . into () } fn json_schema (_ : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : $ instance_type }) } } } ; ($ type : ty => $ instance_type : literal , $ format : literal) => { impl JsonSchema for $ type { inline_schema ! () ; fn schema_name () -> Cow <'static , str > { $ format . into () } fn json_schema (_ : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : $ instance_type , "format" : $ format }) } } } ; }
};
}
