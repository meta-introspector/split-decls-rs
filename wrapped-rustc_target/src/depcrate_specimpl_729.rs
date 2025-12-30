// Generated macro for impl_729 (impl)
macro_rules! Depcrate_specimpl_729 {
() => {
// Module: crate::spec
// Provides: {"impl_729"}
// Dependencies: {}
impl schemars :: JsonSchema for LinkSelfContainedComponents { fn schema_name () -> std :: borrow :: Cow < 'static , str > { "LinkSelfContainedComponents" . into () } fn json_schema (_ : & mut schemars :: SchemaGenerator) -> schemars :: Schema { let all = Self :: all_components () . iter () . map (| component | component . as_str ()) . collect :: < Vec < _ > > () ; schemars :: json_schema ! ({ "type" : "string" , "enum" : all , }) . into () } }
};
}
