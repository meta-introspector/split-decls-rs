// Generated macro for impl_698 (impl)
macro_rules! Depcrate_spec_jsonimpl_698 {
() => {
// Module: crate::spec::json
// Provides: {"impl_698"}
// Dependencies: {}
impl schemars :: JsonSchema for ExternAbiWrapper { fn schema_name () -> std :: borrow :: Cow < 'static , str > { "ExternAbi" . into () } fn json_schema (_ : & mut schemars :: SchemaGenerator) -> schemars :: Schema { let all = rustc_abi :: ExternAbi :: ALL_VARIANTS . iter () . map (| abi | abi . as_str ()) . collect :: < Vec < _ > > () ; schemars :: json_schema ! ({ "type" : "string" , "enum" : all , }) . into () } }
};
}
