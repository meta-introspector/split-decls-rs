// Generated macro for test_basic (function)
macro_rules! Depcrate_locale_parentstest_basic {
() => {
// Module: crate::locale::parents
// Provides: {"test_basic"}
// Dependencies: {}
# [test] fn test_basic () { use icu :: locale :: { langid , LanguageIdentifier } ; let provider = SourceDataProvider :: new_testing () ; let parents : DataResponse < LocaleParentsV1 > = provider . load (Default :: default ()) . unwrap () ; assert_eq ! (parents . payload . get () . parents . get_copied ("zh-Hant-MO" . into ()) . map (LanguageIdentifier :: from) , Some (langid ! ("zh-Hant-HK"))) ; }
};
}
