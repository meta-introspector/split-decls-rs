// Generated macro for EnumDiscriminantsMeta (enum)
macro_rules! Depcrate_helpers_metadataEnumDiscriminantsMeta {
() => {
// Module: crate::helpers::metadata
// Provides: {"EnumDiscriminantsMeta"}
// Dependencies: {}
pub enum EnumDiscriminantsMeta { Derive { _kw : kw :: derive , paths : Vec < Path > } , Name { kw : kw :: name , name : Ident } , Vis { kw : kw :: vis , vis : Visibility } , Doc { _kw : kw :: doc , doc : LitStr } , Other { path : Path , nested : TokenStream } , }
};
}
