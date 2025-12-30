// Generated macro for test_missing_append_items_support (function)
macro_rules! Depcrate_datetime_legacytest_missing_append_items_support {
() => {
// Module: crate::datetime::legacy
// Provides: {"test_missing_append_items_support"}
// Dependencies: {}
# [test] # [should_panic] fn test_missing_append_items_support () { let mut components = components :: Bag :: empty () ; components . year = Some (components :: Year :: Numeric) ; components . month = Some (components :: Month :: Long) ; components . day = Some (components :: Day :: NumericDayOfMonth) ; components . time_zone_name = Some (components :: TimeZoneName :: LongSpecific) ; let requested_fields = components . to_vec_fields (HourCycle :: H23) ; let provider = SourceDataProvider :: new_testing () ; let data = provider . get_dates_resource (& langid ! ("en") . into () , Some (DatagenCalendar :: Gregorian)) . unwrap () ; let patterns = DateLengths :: from (data) ; let skeletons = data . datetime_formats . available_formats . parse_skeletons () ; match create_best_pattern_for_fields (& skeletons , & patterns . length_combinations , & requested_fields , & Default :: default () , false ,) { BestSkeleton :: AllFieldsMatch (available_format_pattern , _) => { assert_eq ! (available_format_pattern . try_into_other () . expect ("pattern should not have plural variants") . to_string () . as_str () , "MMMM d, y vvvv") } best => panic ! ("Unexpected {best:?}") , } ; }
};
}
