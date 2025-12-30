// Generated macro for test_basic (function)
macro_rules! Depcrate_pluralstest_basic {
() => {
// Module: crate::plurals
// Provides: {"test_basic"}
// Dependencies: {}
# [test] fn test_basic () { use icu :: locale :: langid ; let provider = SourceDataProvider :: new_testing () ; let cs_rules : DataResponse < PluralsCardinalV1 > = provider . load (DataRequest { id : DataIdentifierCow :: from_locale (langid ! ("cs") . into ()) . as_borrowed () , .. Default :: default () }) . unwrap () ; assert_eq ! (None , cs_rules . payload . get () . zero) ; assert_eq ! (Some ("i = 1 and v = 0" . parse () . expect ("Failed to parse rule")) , cs_rules . payload . get () . one) ; assert_eq ! (None , cs_rules . payload . get () . two) ; assert_eq ! (Some ("i = 2..4 and v = 0" . parse () . expect ("Failed to parse rule")) , cs_rules . payload . get () . few) ; assert_eq ! (Some ("v != 0" . parse () . expect ("Failed to parse rule")) , cs_rules . payload . get () . many) ; }
};
}
