// Generated macro for test_basic (function)
macro_rules! Depcrate_currency_extendedtest_basic {
() => {
// Module: crate::currency::extended
// Provides: {"test_basic"}
// Dependencies: {}
# [test] fn test_basic () { use icu :: locale :: langid ; use icu :: plurals :: PluralRules ; let provider = SourceDataProvider :: new_testing () ; let en : DataPayload < CurrencyExtendedDataV1 > = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_marker_attributes_and_locale (DataMarkerAttributes :: from_str_or_panic ("USD") , & langid ! ("en") . into () ,) , .. Default :: default () }) . unwrap () . payload ; let en_rules = PluralRules :: try_new_cardinal_unstable (& provider , langid ! ("en") . into ()) . unwrap () ; assert_eq ! (en . get () . display_names . get (1 . into () , & en_rules) , "US dollar") ; assert_eq ! (en . get () . display_names . get (10 . into () , & en_rules) , "US dollars") ; let fr : DataPayload < CurrencyExtendedDataV1 > = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_marker_attributes_and_locale (DataMarkerAttributes :: from_str_or_panic ("USD") , & langid ! ("fr") . into () ,) , .. Default :: default () }) . unwrap () . payload ; let fr_rules = PluralRules :: try_new_cardinal_unstable (& provider , langid ! ("fr") . into ()) . unwrap () ; assert_eq ! (fr . get () . display_names . get (0 . into () , & fr_rules) , "dollar des États-Unis") ; assert_eq ! (fr . get () . display_names . get (1 . into () , & fr_rules) , "dollar des États-Unis") ; assert_eq ! (fr . get () . display_names . get (10 . into () , & fr_rules) , "dollars des États-Unis") ; }
};
}
