// Generated macro for impl_1131 (impl)
macro_rules! Depcrate_segmenter_dictionaryimpl_1131 {
() => {
// Module: crate::segmenter::dictionary
// Provides: {"impl_1131"}
// Dependencies: {}
impl SourceDataProvider { fn load_dictionary_data (& self , req : DataRequest ,) -> Result < UCharDictionaryBreakData < 'static > , DataError > { let filename = format ! ("segmenter/dictionary/{}.toml" , req . id . marker_attributes as & str) ; let toml_data = self . icuexport () . and_then (| e | e . read_and_parse_toml :: < SegmenterDictionaryData > (& filename)) ; Ok (UCharDictionaryBreakData { trie_data : ZeroVec :: alloc_from_slice (& toml_data ? . trie_data) , }) } }
};
}
