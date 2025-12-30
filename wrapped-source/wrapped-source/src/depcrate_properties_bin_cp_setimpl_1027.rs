// Generated macro for impl_1027 (impl)
macro_rules! Depcrate_properties_bin_cp_setimpl_1027 {
() => {
// Module: crate::properties::bin_cp_set
// Provides: {"impl_1027"}
// Dependencies: {}
impl SourceDataProvider { pub (super) fn get_binary_prop_for_code_point_set < 'a > (& 'a self , key : & str ,) -> Result < & 'a super :: uprops_serde :: binary :: BinaryProperty , DataError > { self . icuexport () ? . read_and_parse_toml :: < super :: uprops_serde :: binary :: Main > (& format ! ("uprops/{}/{}.toml" , self . trie_type () , key)) ? . binary_property . first () . ok_or_else (| | DataErrorKind :: MarkerNotFound . into_error ()) } }
};
}
