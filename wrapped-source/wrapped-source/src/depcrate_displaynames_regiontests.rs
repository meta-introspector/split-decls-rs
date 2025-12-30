// Generated macro for tests (module)
macro_rules! Depcrate_displaynames_regiontests {
() => {
// Module: crate::displaynames::region
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use icu :: locale :: { langid , subtags :: region } ; # [test] fn test_basic () { let provider = SourceDataProvider :: new_testing () ; let data : DataPayload < RegionDisplayNamesV1 > = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_locale (& langid ! ("en-001") . into ()) , .. Default :: default () }) . unwrap () . payload ; assert_eq ! (data . get () . names . get (& region ! ("AE") . to_tinystr () . to_unvalidated ()) . unwrap () , "United Arab Emirates") ; } # [test] fn test_basic_short_names () { let provider = SourceDataProvider :: new_testing () ; let data : DataPayload < RegionDisplayNamesV1 > = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_locale (& langid ! ("en-001") . into ()) , .. Default :: default () }) . unwrap () . payload ; assert_eq ! (data . get () . short_names . get (& region ! ("BA") . to_tinystr () . to_unvalidated ()) . unwrap () , "Bosnia") ; } }
};
}
