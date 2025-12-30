// Generated macro for test_en_hour_patterns (function)
macro_rules! Depcrate_datetime_neo_skeletontest_en_hour_patterns {
() => {
// Module: crate::datetime::neo_skeleton
// Provides: {"test_en_hour_patterns"}
// Dependencies: {}
# [test] fn test_en_hour_patterns () { use icu :: locale :: locale ; let provider = SourceDataProvider :: new_testing () ; let payload : DataPayload < DatetimePatternsTimeV1 > = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_marker_attributes_and_locale (DataMarkerAttributes :: from_str_or_panic ("j") , & locale ! ("en") . into () ,) , metadata : Default :: default () , }) . unwrap () . payload ; let json_str = serde_json :: to_string_pretty (payload . get ()) . unwrap () ; assert_eq ! (json_str , r#"{
  "variant_pattern_indices": [
    2,
    2,
    2,
    3,
    3,
    3
  ],
  "elements": [
    "h a",
    "h:mm a",
    "h:mm:ss a"
  ]
}"#) ; }
};
}
