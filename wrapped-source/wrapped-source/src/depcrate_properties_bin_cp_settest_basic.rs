// Generated macro for test_basic (function)
macro_rules! Depcrate_properties_bin_cp_settest_basic {
() => {
// Module: crate::properties::bin_cp_set
// Provides: {"test_basic"}
// Dependencies: {}
# [test] fn test_basic () { use icu :: properties :: { props :: WhiteSpace , CodePointSetData } ; let provider = SourceDataProvider :: new_testing () ; let whitespace = CodePointSetData :: try_new_unstable :: < WhiteSpace > (& provider) . unwrap () ; let whitespace = whitespace . as_code_point_inversion_list () . unwrap () ; assert ! (whitespace . contains (' ')) ; assert ! (whitespace . contains ('\n')) ; assert ! (whitespace . contains ('\u{3000}')) ; assert ! (! whitespace . contains ('A')) ; }
};
}
