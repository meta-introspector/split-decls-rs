// Generated macro for test_basic (function)
macro_rules! Depcrate_currency_compacttest_basic {
() => {
// Module: crate::currency::compact
// Provides: {"test_basic"}
// Dependencies: {}
# [test] fn test_basic () { use icu :: experimental :: dimension :: provider :: currency :: compact :: * ; use icu :: locale :: langid ; let provider = SourceDataProvider :: new_testing () ; let en : DataResponse < ShortCurrencyCompactV1 > = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_locale (& langid ! ("en") . into ()) , .. Default :: default () }) . unwrap () ; let en_patterns = & en . payload . get () . to_owned () . compact_patterns ; assert_eq ! (en_patterns . get (& (3 , CompactCount :: Standard (PluralCategory :: One))) , None) ; assert_eq ! (en_patterns . get (& (3 , CompactCount :: AlphaNextToNumber (PluralCategory :: One))) , None) ; assert_eq ! (en_patterns . get (& (3 , CompactCount :: Standard (PluralCategory :: Other))) , Some ("¤0K")) ; assert_eq ! (en_patterns . get (& (3 , CompactCount :: AlphaNextToNumber (PluralCategory :: Other))) , Some ("¤ 0K")) ; let ja : DataResponse < ShortCurrencyCompactV1 > = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_locale (& langid ! ("ja") . into ()) , .. Default :: default () }) . unwrap () ; let ja_patterns = & ja . payload . get () . to_owned () . compact_patterns ; assert_eq ! (ja_patterns . get (& (4 , CompactCount :: Standard (PluralCategory :: Other))) , Some ("¤0万")) ; assert_eq ! (ja_patterns . get (& (4 , CompactCount :: AlphaNextToNumber (PluralCategory :: Other))) , Some ("¤\u{a0}0万")) ; }
};
}
