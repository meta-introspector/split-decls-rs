// Generated macro for test_basic (function)
macro_rules! Depcrate_locale_likely_subtagstest_basic {
() => {
// Module: crate::locale::likely_subtags
// Provides: {"test_basic"}
// Dependencies: {}
# [test] fn test_basic () { use icu :: locale :: subtags :: { language , region , script } ; let provider = SourceDataProvider :: new_testing () ; let result_common_sr : DataResponse < LocaleLikelySubtagsScriptRegionV1 > = provider . load (Default :: default ()) . unwrap () ; let result_extended : DataResponse < LocaleLikelySubtagsExtendedV1 > = provider . load (Default :: default ()) . unwrap () ; let entry = result_common_sr . payload . get () . script . get_copied (& script ! ("Hant") . to_tinystr () . to_unvalidated ()) . unwrap () ; assert_eq ! (entry . 0 , language ! ("zh")) ; assert_eq ! (entry . 1 , region ! ("TW")) ; let entry = result_extended . payload . get () . script . get_copied (& script ! ("Glag") . to_tinystr () . to_unvalidated ()) . unwrap () ; assert_eq ! (entry . 0 , language ! ("cu")) ; assert_eq ! (entry . 1 , region ! ("BG")) ; }
};
}
