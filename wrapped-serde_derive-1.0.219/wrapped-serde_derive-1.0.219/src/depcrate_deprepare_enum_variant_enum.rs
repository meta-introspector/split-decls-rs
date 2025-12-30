// Generated macro for prepare_enum_variant_enum (function)
macro_rules! Depcrate_deprepare_enum_variant_enum {
() => {
// Module: crate::de
// Provides: {"prepare_enum_variant_enum"}
// Dependencies: {}
fn prepare_enum_variant_enum (variants : & [Variant]) -> (TokenStream , Stmts) { let deserialized_variants = variants . iter () . enumerate () . filter (| & (_i , variant) | ! variant . attrs . skip_deserializing ()) ; let fallthrough = deserialized_variants . clone () . find (| (_i , variant) | variant . attrs . other ()) . map (| (i , _variant) | { let ignore_variant = field_i (i) ; quote ! (_serde :: __private :: Ok (__Field ::# ignore_variant)) }) ; let variants_stmt = { let variant_names = deserialized_variants . clone () . flat_map (| (_i , variant) | variant . attrs . aliases ()) ; quote ! { # [doc (hidden)] const VARIANTS : &'static [&'static str] = & [# (# variant_names) ,*] ; } } ; let deserialized_variants : Vec < _ > = deserialized_variants . map (| (i , variant) | FieldWithAliases { ident : field_i (i) , aliases : variant . attrs . aliases () , }) . collect () ; let variant_visitor = Stmts (deserialize_generated_identifier (& deserialized_variants , false , true , None , fallthrough ,)) ; (variants_stmt , variant_visitor) }
};
}
