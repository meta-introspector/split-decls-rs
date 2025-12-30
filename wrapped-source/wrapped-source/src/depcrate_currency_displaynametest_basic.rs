// Generated macro for test_basic (function)
macro_rules! Depcrate_currency_displaynametest_basic {
() => {
// Module: crate::currency::displayname
// Provides: {"test_basic"}
// Dependencies: {}
# [test] fn test_basic () { use icu :: locale :: langid ; let provider = SourceDataProvider :: new_testing () ; let en : DataPayload < CurrencyDisplaynameV1 > = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_marker_attributes_and_locale (DataMarkerAttributes :: from_str_or_panic ("USD") , & langid ! ("en") . into () ,) , .. Default :: default () }) . unwrap () . payload ; let display_name = en . get () . to_owned () . display_name ; assert_eq ! (display_name , "US Dollar") ; let fr : DataPayload < CurrencyDisplaynameV1 > = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_marker_attributes_and_locale (DataMarkerAttributes :: from_str_or_panic ("USD") , & langid ! ("fr") . into () ,) , .. Default :: default () }) . unwrap () . payload ; let display_name = fr . get () . to_owned () . display_name ; assert_eq ! (display_name , "dollar des États-Unis") ; }
};
}
