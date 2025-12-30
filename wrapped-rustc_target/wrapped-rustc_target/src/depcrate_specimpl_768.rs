// Generated macro for impl_768 (impl)
macro_rules! Depcrate_specimpl_768 {
() => {
// Module: crate::spec
// Provides: {"impl_768"}
// Dependencies: {}
impl schemars :: JsonSchema for SanitizerSet { fn schema_name () -> std :: borrow :: Cow < 'static , str > { "SanitizerSet" . into () } fn json_schema (_ : & mut schemars :: SchemaGenerator) -> schemars :: Schema { let all = Self :: all () . iter () . map (| sanitizer | sanitizer . as_str ()) . collect :: < Vec < _ > > () ; schemars :: json_schema ! ({ "type" : "string" , "enum" : all , }) . into () } }
};
}
