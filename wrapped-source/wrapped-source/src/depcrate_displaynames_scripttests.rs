// Generated macro for tests (module)
macro_rules! Depcrate_displaynames_scripttests {
() => {
// Module: crate::displaynames::script
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use icu :: locale :: { langid , subtags :: script } ; # [test] fn test_basic_script_display_names () { let provider = SourceDataProvider :: new_testing () ; let data : DataPayload < ScriptDisplayNamesV1 > = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_locale (& langid ! ("en-001") . into ()) , .. Default :: default () }) . unwrap () . payload ; assert_eq ! (data . get () . names . get (& script ! ("Cans") . to_tinystr () . to_unvalidated ()) . unwrap () , "Unified Canadian Aboriginal Syllabics") ; } # [test] fn test_basic_script_short_display_names () { let provider = SourceDataProvider :: new_testing () ; let data : DataPayload < ScriptDisplayNamesV1 > = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_locale (& langid ! ("en-001") . into ()) , .. Default :: default () }) . unwrap () . payload ; assert_eq ! (data . get () . short_names . get (& script ! ("Cans") . to_tinystr () . to_unvalidated ()) . unwrap () , "UCAS") ; } }
};
}
