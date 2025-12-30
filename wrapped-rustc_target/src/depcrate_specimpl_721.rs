// Generated macro for impl_721 (impl)
macro_rules! Depcrate_specimpl_721 {
() => {
// Module: crate::spec
// Provides: {"impl_721"}
// Dependencies: {}
impl schemars :: JsonSchema for LinkSelfContainedDefault { fn schema_name () -> std :: borrow :: Cow < 'static , str > { "LinkSelfContainedDefault" . into () } fn json_schema (_ : & mut schemars :: SchemaGenerator) -> schemars :: Schema { schemars :: json_schema ! ({ "type" : "string" , "enum" : ["false" , "true" , "wasm" , "musl" , "mingw"] }) . into () } }
};
}
