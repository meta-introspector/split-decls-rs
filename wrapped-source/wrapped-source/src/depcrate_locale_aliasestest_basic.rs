// Generated macro for test_basic (function)
macro_rules! Depcrate_locale_aliasestest_basic {
() => {
// Module: crate::locale::aliases
// Provides: {"test_basic"}
// Dependencies: {}
# [test] fn test_basic () { use icu :: locale :: subtags :: { language , region , script } ; let provider = SourceDataProvider :: new_testing () ; let data : DataResponse < LocaleAliasesV1 > = provider . load (Default :: default ()) . unwrap () ; assert ! (data . payload . get () . language . is_empty ()) ; assert ! (! data . payload . get () . language_variants . is_empty ()) ; assert ! (! data . payload . get () . sgn_region . is_empty ()) ; assert ! (! data . payload . get () . language_len2 . is_empty ()) ; assert ! (! data . payload . get () . language_len3 . is_empty ()) ; assert ! (! data . payload . get () . script . is_empty ()) ; assert ! (! data . payload . get () . region_alpha . is_empty ()) ; assert ! (! data . payload . get () . region_num . is_empty ()) ; assert ! (! data . payload . get () . complex_region . is_empty ()) ; assert ! (! data . payload . get () . variant . is_empty ()) ; assert ! (! data . payload . get () . subdivision . is_empty ()) ; assert_eq ! (data . payload . get () . language_len2 . get (& language ! ("iw") . to_tinystr () . resize () . to_unvalidated ()) . unwrap () , "he") ; assert ! (data . payload . get () . language_len3 . get (& language ! ("iw") . to_tinystr () . to_unvalidated ()) . is_none ()) ; assert_eq ! (data . payload . get () . script . iter () . next () . unwrap () , (& script ! ("Qaai") . to_tinystr () . to_unvalidated () , & script ! ("Zinh"))) ; assert_eq ! (data . payload . get () . region_num . get (& region ! ("768") . to_tinystr () . to_unvalidated ()) . unwrap () , & region ! ("TG")) ; }
};
}
