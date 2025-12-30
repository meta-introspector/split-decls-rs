// Generated macro for tests (module)
macro_rules! Depcrate_transformstests {
() => {
// Module: crate::transforms
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_de_ascii_forward () { let provider = SourceDataProvider :: new_testing () ; let _data : DataPayload < TransliteratorRulesV1 > = provider . load (DataRequest { id : DataIdentifierCow :: from_marker_attributes (DataMarkerAttributes :: from_str_or_panic ("de-t-de-d0-ascii") ,) . as_borrowed () , .. Default :: default () }) . unwrap () . payload ; } # [test] fn test_latin_ascii_backward () { let provider = SourceDataProvider :: new_testing () ; let _data : DataPayload < TransliteratorRulesV1 > = provider . load (DataRequest { id : DataIdentifierCow :: from_marker_attributes (DataMarkerAttributes :: from_str_or_panic ("und-latn-t-s0-ascii") ,) . as_borrowed () , .. Default :: default () }) . unwrap () . payload ; } }
};
}
