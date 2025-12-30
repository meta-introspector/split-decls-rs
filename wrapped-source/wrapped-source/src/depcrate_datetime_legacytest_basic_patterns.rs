// Generated macro for test_basic_patterns (function)
macro_rules! Depcrate_datetime_legacytest_basic_patterns {
() => {
// Module: crate::datetime::legacy
// Provides: {"test_basic_patterns"}
// Dependencies: {}
# [test] fn test_basic_patterns () { let provider = SourceDataProvider :: new_testing () ; let data = provider . get_dates_resource (& langid ! ("cs") . into () , Some (DatagenCalendar :: Gregorian)) . unwrap () ; let cs_dates = DateLengths :: from (data) ; assert_eq ! ("yMd" , cs_dates . date . medium . to_string ()) ; }
};
}
