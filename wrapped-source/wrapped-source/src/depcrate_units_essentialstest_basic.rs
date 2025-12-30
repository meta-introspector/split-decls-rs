// Generated macro for test_basic (function)
macro_rules! Depcrate_units_essentialstest_basic {
() => {
// Module: crate::units::essentials
// Provides: {"test_basic"}
// Dependencies: {}
# [test] fn test_basic () { use icu :: locale :: langid ; use icu_provider :: prelude :: * ; let provider = SourceDataProvider :: new_testing () ; let us_long : DataPayload < UnitsEssentialsV1 > = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_marker_attributes_and_locale (DataMarkerAttributes :: from_str_or_panic ("long") , & langid ! ("en") . into () ,) , .. Default :: default () }) . unwrap () . payload ; let per = us_long . get () . per . to_string () ; assert_eq ! (per , "{0} per {1}") ; let times = us_long . get () . times . to_string () ; assert_eq ! (times , "{0}-{1}") ; let fr_long : DataPayload < UnitsEssentialsV1 > = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_marker_attributes_and_locale (DataMarkerAttributes :: from_str_or_panic ("long") , & langid ! ("fr") . into () ,) , .. Default :: default () }) . unwrap () . payload ; let per = fr_long . get () . per . to_string () ; assert_eq ! (per , "{0} par {1}") ; let times = fr_long . get () . times . to_string () ; assert_eq ! (times , "{0}-{1}") ; let ar_eg_short : DataPayload < UnitsEssentialsV1 > = provider . load (DataRequest { id : DataIdentifierBorrowed :: for_marker_attributes_and_locale (DataMarkerAttributes :: from_str_or_panic ("short") , & langid ! ("ar") . into () ,) , .. Default :: default () }) . unwrap () . payload ; let per = ar_eg_short . get () . per . to_string () ; assert_eq ! (per , "{0}/{1}") ; let times = ar_eg_short . get () . times . to_string () ; assert_eq ! (times , "{0}⋅{1}") ; }
};
}
