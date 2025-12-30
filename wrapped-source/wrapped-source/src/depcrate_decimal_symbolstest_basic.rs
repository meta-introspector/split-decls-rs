// Generated macro for test_basic (function)
macro_rules! Depcrate_decimal_symbolstest_basic {
() => {
// Module: crate::decimal::symbols
// Provides: {"test_basic"}
// Dependencies: {}
# [test] fn test_basic () { use icu :: locale :: langid ; let provider = SourceDataProvider :: new_testing () ; let ar_decimal : DataResponse < DecimalSymbolsV1 > = provider . load (DataRequest { id : DataIdentifierCow :: from_locale (langid ! ("ar-EG") . into ()) . as_borrowed () , .. Default :: default () }) . unwrap () ; assert_eq ! (ar_decimal . payload . get () . decimal_separator () , "٫") ; assert_eq ! (ar_decimal . payload . get () . numsys () , "arab") ; }
};
}
