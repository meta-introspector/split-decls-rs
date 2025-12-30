// Generated macro for test_basic (function)
macro_rules! Depcrate_locale_directionalitytest_basic {
() => {
// Module: crate::locale::directionality
// Provides: {"test_basic"}
// Dependencies: {}
# [test] fn test_basic () { use icu :: locale :: subtags :: script ; let provider = SourceDataProvider :: new_testing () ; let data : DataResponse < LocaleScriptDirectionV1 > = provider . load (Default :: default ()) . unwrap () ; assert ! (data . payload . get () . rtl . binary_search (& script ! ("Avst") . to_tinystr () . to_unvalidated ()) . is_ok ()) ; assert ! (data . payload . get () . ltr . binary_search (& script ! ("Avst") . to_tinystr () . to_unvalidated ()) . is_err ()) ; assert ! (data . payload . get () . ltr . binary_search (& script ! ("Latn") . to_tinystr () . to_unvalidated ()) . is_ok ()) ; assert ! (data . payload . get () . rtl . binary_search (& script ! ("Latn") . to_tinystr () . to_unvalidated ()) . is_err ()) ; assert ! (data . payload . get () . ltr . binary_search (& script ! ("Zzzz") . to_tinystr () . to_unvalidated ()) . is_err ()) ; assert ! (data . payload . get () . rtl . binary_search (& script ! ("Zzzz") . to_tinystr () . to_unvalidated ()) . is_err ()) ; }
};
}
