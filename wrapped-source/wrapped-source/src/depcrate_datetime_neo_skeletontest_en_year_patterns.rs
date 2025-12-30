// Generated macro for test_en_year_patterns (function)
macro_rules! Depcrate_datetime_neo_skeletontest_en_year_patterns {
() => {
// Module: crate::datetime::neo_skeleton
// Provides: {"test_en_year_patterns"}
// Dependencies: {}
# [test] fn test_en_year_patterns () { use icu :: locale :: locale ; let provider = SourceDataProvider :: new_testing () ; let payload : DataPayload < DatetimePatternsDateGregorianV1 > = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_marker_attributes_and_locale (DataMarkerAttributes :: from_str_or_panic ("ym0d") , & locale ! ("en") . into () ,) , metadata : Default :: default () , }) . unwrap () . payload ; let json_str = serde_json :: to_string_pretty (payload . get ()) . unwrap () ; assert_eq ! (json_str , r#"{
  "has_explicit_medium": true,
  "has_explicit_short": true,
  "variant_pattern_indices": [
    0,
    0,
    4,
    5,
    6,
    7
  ],
  "elements": [
    "MMMM d, y",
    "MMM d, y",
    "M/d/yy",
    "M/d/y",
    "MMMM d, y GGG",
    "MMM d, y GGG",
    "M/d/y GGG"
  ]
}"#) ; }
};
}
