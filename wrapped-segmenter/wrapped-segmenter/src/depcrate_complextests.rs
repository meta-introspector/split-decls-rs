// Generated macro for tests (module)
macro_rules! Depcrate_complextests {
() => {
// Module: crate::complex
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "serde")] mod tests { use super :: * ; # [test] fn thai_word_break () { const TEST_STR : & str = "ภาษาไทยภาษาไทย" ; let utf16 : Vec < u16 > = TEST_STR . encode_utf16 () . collect () ; let lstm = ComplexPayloadsBorrowed :: new_lstm () ; let dict = ComplexPayloadsBorrowed :: new_dict () ; assert_eq ! (lstm . complex_language_segment_str (TEST_STR) , [12 , 21 , 33 , 42]) ; assert_eq ! (lstm . complex_language_segment_utf16 (& utf16) , [4 , 7 , 11 , 14]) ; assert_eq ! (dict . complex_language_segment_str (TEST_STR) , [12 , 21 , 33 , 42]) ; assert_eq ! (dict . complex_language_segment_utf16 (& utf16) , [4 , 7 , 11 , 14]) ; } }
};
}
