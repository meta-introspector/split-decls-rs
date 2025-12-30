// Generated macro for impl_275 (impl)
macro_rules! Depcrate_schemaimpl_275 {
() => {
// Module: crate::schema
// Provides: {"impl_275"}
// Dependencies: {}
impl crate :: JsonSchema for Schema { fn schema_name () -> alloc :: borrow :: Cow < 'static , str > { "Schema" . into () } fn schema_id () -> alloc :: borrow :: Cow < 'static , str > { "schemars::Schema" . into () } fn json_schema (_ : & mut crate :: SchemaGenerator) -> Schema { crate :: json_schema ! ({ "type" : ["object" , "boolean"] }) } }
};
}
