// Generated macro for impl_1053 (impl)
macro_rules! Depcrate_properties_enum_codepointtrieimpl_1053 {
() => {
// Module: crate::properties::enum_codepointtrie
// Provides: {"impl_1053"}
// Dependencies: {}
impl SourceDataProvider { pub (super) fn get_enumerated_prop < 'a > (& 'a self , key : & str ,) -> Result < & 'a super :: uprops_serde :: enumerated :: EnumeratedPropertyMap , DataError > { self . icuexport () ? . read_and_parse_toml :: < super :: uprops_serde :: enumerated :: Main > (& format ! ("uprops/{}/{}.toml" , self . trie_type () , key)) ? . enum_property . first () . ok_or_else (| | DataErrorKind :: MarkerNotFound . into_error ()) } fn get_mask_prop < 'a > (& 'a self , key : & str ,) -> Result < & 'a super :: uprops_serde :: mask :: MaskPropertyMap , DataError > { self . icuexport () ? . read_and_parse_toml :: < super :: uprops_serde :: mask :: Main > (& format ! ("uprops/{}/{}.toml" , self . trie_type () , key)) ? . mask_property . first () . ok_or (DataError :: custom ("Loading icuexport property data failed: \
                 Are you using a sufficiently recent icuexport? (Must be ⪈ 72.1)" ,)) } }
};
}
