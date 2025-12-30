// Generated macro for impl_1304 (impl)
macro_rules! Depcrate_ucaseimpl_1304 {
() => {
// Module: crate::ucase
// Provides: {"impl_1304"}
// Dependencies: {}
impl DataProvider < CaseMapV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < CaseMapV1 > , DataError > { self . check_req :: < CaseMapV1 > (req) ? ; let toml = & self . icuexport () ? . read_and_parse_toml :: < ucase_serde :: Main > (& format ! ("ucase/{}/ucase.toml" , self . trie_type ())) ? . ucase ; let trie_data = & toml . code_point_trie ; let trie_header = CodePointTrieHeader :: try_from (trie_data) . map_err (| e | { DataError :: custom ("Could not parse CodePointTrie TOML") . with_display_context (& e) }) ? ; let trie_index = trie_data . index_slice () ; let trie_data = if let Ok (CodePointDataSlice :: U16 (s)) = trie_data . data_slice () { s } else { return Err (DataError :: custom ("Did not find 16-bit data array for case mapping in TOML" ,)) ; } ; let exceptions = & toml . exceptions . exceptions ; let case_mapping = CaseMap :: try_from_icu (trie_header , trie_index , trie_data , exceptions) ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (case_mapping) , }) } }
};
}
