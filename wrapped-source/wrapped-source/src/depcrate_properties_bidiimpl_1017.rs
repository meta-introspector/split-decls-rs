// Generated macro for impl_1017 (impl)
macro_rules! Depcrate_properties_bidiimpl_1017 {
() => {
// Module: crate::properties::bidi
// Provides: {"impl_1017"}
// Dependencies: {}
# [cfg (any (feature = "use_wasm" , feature = "use_icu4c"))] impl SourceDataProvider { fn get_code_point_prop_map < 'a > (& 'a self , key : & str ,) -> Result < & 'a super :: uprops_serde :: code_point_prop :: CodePointPropertyMap , DataError > { self . icuexport () ? . read_and_parse_toml :: < super :: uprops_serde :: code_point_prop :: Main > (& format ! ("uprops/{}/{}.toml" , self . trie_type () , key)) ? . enum_property . first () . ok_or_else (| | DataErrorKind :: MarkerNotFound . into_error ()) } }
};
}
