// Generated macro for tests (module)
macro_rules! Depcrate_displaynames_varianttests {
() => {
// Module: crate::displaynames::variant
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use icu :: locale :: { langid , subtags :: variant } ; # [test] fn test_basic_variant_display_names () { let provider = SourceDataProvider :: new_testing () ; let data : DataPayload < VariantDisplayNamesV1 > = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_locale (& langid ! ("en-001") . into ()) , .. Default :: default () }) . unwrap () . payload ; assert_eq ! (data . get () . names . get (& variant ! ("POSIX") . to_tinystr () . to_unvalidated ()) . unwrap () , "Computer") ; } }
};
}
