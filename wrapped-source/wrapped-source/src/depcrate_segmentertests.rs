// Generated macro for tests (module)
macro_rules! Depcrate_segmentertests {
() => {
// Module: crate::segmenter
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn load_grapheme_cluster_data () { let provider = SourceDataProvider :: new_testing () ; let response : DataResponse < SegmenterBreakGraphemeClusterV1 > = provider . load (Default :: default ()) . expect ("Loading should succeed!") ; assert_eq ! (response . payload . get () . complex_property , 127 , "Grapheme cluster data doesn't handle SA") ; } # [test] fn load_line_data () { let provider = SourceDataProvider :: new_testing () ; let response : DataResponse < SegmenterBreakLineV1 > = provider . load (Default :: default ()) . expect ("Loading should succeed!") ; let data = response . payload . get () ; const CM : u8 = 14 ; const XX : u8 = 52 ; const ID : u8 = 25 ; assert_eq ! (data . property_table . get32 (0x20000) , ID) ; assert_eq ! (data . property_table . get32 (0x3fffd) , ID) ; assert_eq ! (data . property_table . get32 (0xd0000) , XX) ; assert_eq ! (data . property_table . get32 (0xe0001) , CM) ; assert_eq ! (data . property_table . get32 (0xe0020) , CM) ; } # [test] # [should_panic] fn missing_locale_data () { let provider = SourceDataProvider :: new_testing () ; let response : DataResponse < SegmenterBreakSentenceOverrideV1 > = provider . load (Default :: default ()) . expect ("Loading should succeed!") ; response . payload . get () ; } }
};
}
