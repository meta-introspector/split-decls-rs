// Generated macro for test_with_numbering_system (function)
macro_rules! Depcrate_datetime_legacytest_with_numbering_system {
() => {
// Module: crate::datetime::legacy
// Provides: {"test_with_numbering_system"}
// Dependencies: {}
# [test] fn test_with_numbering_system () { let provider = SourceDataProvider :: new_testing () ; let data = provider . get_dates_resource (& langid ! ("haw") . into () , Some (DatagenCalendar :: Gregorian)) . unwrap () ; let haw_dates = DateLengths :: from (data) ; assert_eq ! ("yMMMd" , haw_dates . date . medium . to_string ()) ; assert_eq ! ("yyMd" , haw_dates . date . short . to_string ()) ; }
};
}
