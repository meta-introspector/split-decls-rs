// Generated macro for tests (module)
macro_rules! Depcrate_segmenter_lstmtests {
() => {
// Module: crate::segmenter::lstm
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use icu :: segmenter :: LineSegmenter ; # [test] fn thai_word_break_with_grapheme_model () { struct OverrideProvider (SourceDataProvider) ; impl < M : DataMarker > DataProvider < M > for OverrideProvider where SourceDataProvider : DataProvider < M > , { fn load (& self , req : DataRequest) -> Result < DataResponse < M > , DataError > { if SegmenterLstmAutoV1 :: INFO == M :: INFO { return Ok (DataResponse { payload : DataPayload :: < SegmenterLstmAutoV1 > :: from_owned (self . 0 . segmenter_lstm () . unwrap () . read_and_parse_json :: < RawLstmData > ("Thai_graphclust_model4_heavy/weights.json" ,) . unwrap () . try_convert () . unwrap () ,) . dynamic_cast () ? , metadata : Default :: default () , }) ; } self . 0 . load (req) } } let provider = OverrideProvider (SourceDataProvider :: new_testing ()) ; let segmenter = LineSegmenter :: try_new_lstm_unstable (& provider , Default :: default ()) . unwrap () ; let segmenter = segmenter . as_borrowed () ; const TEST_STR : & str = "ภาษาไทยภาษาไทย" ; let utf16 : Vec < u16 > = TEST_STR . encode_utf16 () . collect () ; let breaks : Vec < usize > = segmenter . segment_str (TEST_STR) . collect () ; assert_eq ! (breaks , [0 , 6 , 12 , 21 , 27 , 33 , TEST_STR . len ()] ,) ; let breaks : Vec < usize > = segmenter . segment_utf16 (& utf16) . collect () ; assert_eq ! (breaks , [0 , 2 , 4 , 7 , 9 , 11 , utf16 . len ()] ,) ; } }
};
}
