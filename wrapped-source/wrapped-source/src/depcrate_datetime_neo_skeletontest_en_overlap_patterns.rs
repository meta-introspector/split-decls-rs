// Generated macro for test_en_overlap_patterns (function)
macro_rules! Depcrate_datetime_neo_skeletontest_en_overlap_patterns {
() => {
// Module: crate::datetime::neo_skeleton
// Provides: {"test_en_overlap_patterns"}
// Dependencies: {}
# [test] fn test_en_overlap_patterns () { use icu :: locale :: locale ; let provider = SourceDataProvider :: new_testing () ; let payload : DataPayload < DatetimePatternsDateGregorianV1 > = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_marker_attributes_and_locale (DataMarkerAttributes :: from_str_or_panic ("ej") , & locale ! ("en") . into () ,) , metadata : Default :: default () , }) . unwrap () . payload ; let json_str = serde_json :: to_string_pretty (payload . get ()) . unwrap () ; assert_eq ! (json_str , r#"{
  "has_explicit_medium": true,
  "variant_pattern_indices": [
    3,
    4,
    4,
    5,
    6,
    6
  ],
  "elements": [
    "EEEE h a",
    "E h a",
    "EEEE h:m a",
    "E h:mm a",
    "EEEE h:m:s a",
    "E h:mm:ss a"
  ]
}"#) ; }
};
}
