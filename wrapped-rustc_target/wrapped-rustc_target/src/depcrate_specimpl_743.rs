// Generated macro for impl_743 (impl)
macro_rules! Depcrate_specimpl_743 {
() => {
// Module: crate::spec
// Provides: {"impl_743"}
// Dependencies: {}
impl schemars :: JsonSchema for SmallDataThresholdSupport { fn schema_name () -> std :: borrow :: Cow < 'static , str > { "SmallDataThresholdSupport" . into () } fn json_schema (_ : & mut schemars :: SchemaGenerator) -> schemars :: Schema { schemars :: json_schema ! ({ "type" : "string" , "pattern" : r#"^none|default-for-arch|llvm-module-flag=.+|llvm-arg=.+$"# , }) . into () } }
};
}
