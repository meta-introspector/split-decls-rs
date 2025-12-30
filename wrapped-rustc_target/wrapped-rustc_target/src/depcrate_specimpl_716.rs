// Generated macro for impl_716 (impl)
macro_rules! Depcrate_specimpl_716 {
() => {
// Module: crate::spec
// Provides: {"impl_716"}
// Dependencies: {}
impl schemars :: JsonSchema for LinkerFlavorCli { fn schema_name () -> std :: borrow :: Cow < 'static , str > { "LinkerFlavor" . into () } fn json_schema (_ : & mut schemars :: SchemaGenerator) -> schemars :: Schema { let all : Vec < & 'static str > = Self :: all () . iter () . map (| flavor | flavor . desc ()) . collect :: < Vec < _ > > () ; schemars :: json_schema ! ({ "type" : "string" , "enum" : all }) . into () } }
};
}
