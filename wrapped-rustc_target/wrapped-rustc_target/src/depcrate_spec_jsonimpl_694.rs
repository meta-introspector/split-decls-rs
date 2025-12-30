// Generated macro for impl_694 (impl)
macro_rules! Depcrate_spec_jsonimpl_694 {
() => {
// Module: crate::spec::json
// Provides: {"impl_694"}
// Dependencies: {}
impl schemars :: JsonSchema for EndianWrapper { fn schema_name () -> std :: borrow :: Cow < 'static , str > { "Endian" . into () } fn json_schema (_ : & mut schemars :: SchemaGenerator) -> schemars :: Schema { schemars :: json_schema ! ({ "type" : "string" , "enum" : ["big" , "little"] }) . into () } }
};
}
