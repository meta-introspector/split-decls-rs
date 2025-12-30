// Generated macro for impl_1063 (impl)
macro_rules! Depcrate_properties_enum_codepointtrieimpl_1063 {
() => {
// Module: crate::properties::enum_codepointtrie
// Provides: {"impl_1063"}
// Dependencies: {}
impl DataProvider < PropertyEnumIndicConjunctBreakV1 > for SourceDataProvider { fn load (& self , req : DataRequest ,) -> Result < DataResponse < PropertyEnumIndicConjunctBreakV1 > , DataError > { self . check_req :: < PropertyEnumIndicConjunctBreakV1 > (req) ? ; let source_cpt_data = & self . get_enumerated_prop ("InCB") ? . code_point_trie ; let code_point_trie = CodePointTrie :: try_from (source_cpt_data) . map_err (| e | { DataError :: custom ("Could not parse CodePointTrie TOML") . with_display_context (& e) }) ? ; let data_struct = PropertyCodePointMap :: CodePointTrie (code_point_trie) ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (data_struct) , }) } }
};
}
