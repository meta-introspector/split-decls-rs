// Generated macro for test_appendix_c_cmp (function)
macro_rules! Depcrate_locale_aliasestest_appendix_c_cmp {
() => {
// Module: crate::locale::aliases
// Provides: {"test_appendix_c_cmp"}
// Dependencies: {}
# [test] fn test_appendix_c_cmp () { let en = icu :: locale :: langid ! ("en-GB") ; let ca = icu :: locale :: langid ! ("ca") ; let und = "und-hepburn-heploc" . parse :: < LanguageIdentifier > () . unwrap () ; let fr = icu :: locale :: langid ! ("fr-CA") ; let mut rules = vec ! [& en , & ca , & und , & fr] ; rules . sort_unstable_by_key (| & l | appendix_c_cmp (l)) ; assert_eq ! (rules , & [& en , & fr , & und , & ca]) ; }
};
}
