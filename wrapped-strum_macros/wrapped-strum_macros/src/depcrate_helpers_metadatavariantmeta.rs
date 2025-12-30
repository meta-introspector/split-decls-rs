// Generated macro for VariantMeta (enum)
macro_rules! Depcrate_helpers_metadataVariantMeta {
() => {
// Module: crate::helpers::metadata
// Provides: {"VariantMeta"}
// Dependencies: {}
pub enum VariantMeta { Message { kw : kw :: message , value : LitStr , } , DetailedMessage { kw : kw :: detailed_message , value : LitStr , } , Serialize { _kw : kw :: serialize , value : LitStr , } , Documentation { value : LitStr , } , ToString { kw : kw :: to_string , value : LitStr , } , Transparent (kw :: transparent) , Disabled (kw :: disabled) , Default (kw :: default) , DefaultWith { kw : kw :: default_with , value : LitStr , } , AsciiCaseInsensitive { kw : kw :: ascii_case_insensitive , value : bool , } , Props { _kw : kw :: props , props : Vec < (LitStr , Lit) > , } , }
};
}
