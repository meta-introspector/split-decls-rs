// Generated macro for test_basic (function)
macro_rules! Depcrate_percenttest_basic {
() => {
// Module: crate::percent
// Provides: {"test_basic"}
// Dependencies: {}
# [test] fn test_basic () { use icu :: experimental :: dimension :: provider :: percent :: * ; use icu :: locale :: langid ; use writeable :: assert_writeable_eq ; let provider = SourceDataProvider :: new_testing () ; let en : DataResponse < PercentEssentialsV1 > = provider . load (DataRequest { id : DataIdentifierCow :: from_locale (langid ! ("en") . into ()) . as_borrowed () , .. Default :: default () }) . unwrap () ; let en_pattern = en . payload . get () . to_owned () ; assert_writeable_eq ! (en_pattern . unsigned_pattern . interpolate (["123"]) , "123%") ; assert_writeable_eq ! (en_pattern . signed_pattern . interpolate (["123" , "+"]) , "+123%") ; let tr : DataResponse < PercentEssentialsV1 > = provider . load (DataRequest { id : DataIdentifierCow :: from_locale (langid ! ("tr") . into ()) . as_borrowed () , .. Default :: default () }) . unwrap () ; let tr_pattern = tr . payload . get () . to_owned () ; assert_writeable_eq ! (tr_pattern . unsigned_pattern . interpolate (["345"]) , "%345") ; assert_writeable_eq ! (tr_pattern . signed_pattern . interpolate (["345" , "+"]) , "+%345") ; let ar_eg : DataResponse < PercentEssentialsV1 > = provider . load (DataRequest { id : DataIdentifierCow :: from_locale (langid ! ("ar-EG") . into ()) . as_borrowed () , .. Default :: default () }) . unwrap () ; let ar_eg_pattern = ar_eg . payload . get () . to_owned () ; assert_writeable_eq ! (ar_eg_pattern . unsigned_pattern . interpolate (["456"]) , "456\u{200e}%\u{200e}") ; assert_writeable_eq ! (ar_eg_pattern . signed_pattern . interpolate (["456" , "+"]) , "+456\u{200e}%\u{200e}") ; }
};
}
