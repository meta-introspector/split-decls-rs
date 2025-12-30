// Generated macro for impl_1039 (impl)
macro_rules! Depcrate_properties_emoji_setimpl_1039 {
() => {
// Module: crate::properties::emoji_set
// Provides: {"impl_1039"}
// Dependencies: {}
impl SourceDataProvider { fn get_binary_prop_for_unicodeset < 'a > (& 'a self , key : & str ,) -> Result < & 'a super :: uprops_serde :: binary :: BinaryProperty , DataError > { self . icuexport () ? . read_and_parse_toml :: < super :: uprops_serde :: binary :: Main > (& format ! ("uprops/{}/{}.toml" , self . trie_type () , key)) ? . binary_property . first () . ok_or_else (| | DataErrorKind :: MarkerNotFound . into_error ()) } }
};
}
