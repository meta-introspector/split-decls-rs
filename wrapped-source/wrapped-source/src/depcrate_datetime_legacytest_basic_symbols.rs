// Generated macro for test_basic_symbols (function)
macro_rules! Depcrate_datetime_legacytest_basic_symbols {
() => {
// Module: crate::datetime::legacy
// Provides: {"test_basic_symbols"}
// Dependencies: {}
# [test] fn test_basic_symbols () { use icu :: calendar :: types :: MonthCode ; use tinystr :: tinystr ; let provider = SourceDataProvider :: new_testing () ; let data = provider . get_dates_resource (& langid ! ("cs") . into () , Some (DatagenCalendar :: Gregorian)) . unwrap () ; let cs_dates = convert_dates (data , DatagenCalendar :: Gregorian) ; assert_eq ! ("srpna" , cs_dates . months . format . wide . get (MonthCode (tinystr ! (4 , "M08"))) . unwrap ()) ; assert_eq ! ("po" , cs_dates . weekdays . format . short . as_ref () . unwrap () . 0 [1]) ; }
};
}
