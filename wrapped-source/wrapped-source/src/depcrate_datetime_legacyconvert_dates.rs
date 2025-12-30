// Generated macro for convert_dates (function)
macro_rules! Depcrate_datetime_legacyconvert_dates {
() => {
// Module: crate::datetime::legacy
// Provides: {"convert_dates"}
// Dependencies: {}
fn convert_dates (other : & cldr_serde :: ca :: Dates , calendar : DatagenCalendar) -> DateSymbols < 'static > { DateSymbols { months : other . months . get (& ({ static SOLAR_MONTH_CODES : & [TinyStr4] = & [tinystr ! (4 , "M01") , tinystr ! (4 , "M02") , tinystr ! (4 , "M03") , tinystr ! (4 , "M04") , tinystr ! (4 , "M05") , tinystr ! (4 , "M06") , tinystr ! (4 , "M07") , tinystr ! (4 , "M08") , tinystr ! (4 , "M09") , tinystr ! (4 , "M10") , tinystr ! (4 , "M11") , tinystr ! (4 , "M12") , tinystr ! (4 , "M13") ,] ; static HEBREW_MONTH_CODES : & [TinyStr4] = & [tinystr ! (4 , "M01") , tinystr ! (4 , "M02") , tinystr ! (4 , "M03") , tinystr ! (4 , "M04") , tinystr ! (4 , "M05") , tinystr ! (4 , "M05L") , tinystr ! (4 , "M06") , tinystr ! (4 , "M07") , tinystr ! (4 , "M08") , tinystr ! (4 , "M09") , tinystr ! (4 , "M10") , tinystr ! (4 , "M11") , tinystr ! (4 , "M12") ,] ; match calendar { DatagenCalendar :: Buddhist | DatagenCalendar :: Chinese | DatagenCalendar :: Dangi | DatagenCalendar :: Gregorian | DatagenCalendar :: Indian | DatagenCalendar :: Hijri | DatagenCalendar :: JapaneseExtended | DatagenCalendar :: JapaneseModern | DatagenCalendar :: Persian | DatagenCalendar :: Roc => & SOLAR_MONTH_CODES [0 .. 12] , DatagenCalendar :: Coptic | DatagenCalendar :: Ethiopic => SOLAR_MONTH_CODES , DatagenCalendar :: Hebrew => HEBREW_MONTH_CODES , } } , calendar . cldr_name () ,)) , weekdays : other . days . get (& ()) , } }
};
}
