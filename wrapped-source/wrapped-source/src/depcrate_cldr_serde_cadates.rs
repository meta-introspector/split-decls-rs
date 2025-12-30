// Generated macro for Dates (struct)
macro_rules! Depcrate_cldr_serde_caDates {
() => {
// Module: crate::cldr_serde::ca
// Provides: {"Dates"}
// Dependencies: {}
# [doc = " This struct represents a 1:1 mapping of the CLDR ca-gregorian.json data at the key"] # [doc = " \"main.LANGID.dates.calendars.gregorian\" where \"LANGID\" is the identifier."] # [doc = ""] # [doc = " e.g."] # [doc = " <https://github.com/unicode-org/cldr-json/blob/master/cldr-json/cldr-dates-full/main/en/ca-gregorian.json>"] # [derive (PartialEq , Debug , Deserialize , Clone)] pub (crate) struct Dates { pub (crate) months : Contexts < MonthSymbols > , # [serde (rename = "monthPatterns")] pub (crate) month_patterns : Option < Contexts < MonthPatternSymbols > > , pub (crate) days : Contexts < DaySymbols > , pub (crate) eras : Option < Eras > , # [serde (rename = "cyclicNameSets")] pub (crate) cyclic_name_sets : Option < CyclicNameSets > , # [serde (rename = "dayPeriods")] pub (crate) day_periods : Contexts < DayPeriodSymbols > , # [serde (rename = "dateFormats")] pub (crate) date_formats : LengthPatterns , # [serde (rename = "timeFormats")] pub (crate) time_formats : LengthPatterns , # [serde (rename = "dateSkeletons")] pub (crate) date_skeletons : LengthPatterns , # [serde (rename = "timeSkeletons")] pub (crate) time_skeletons : LengthPatterns , # [serde (rename = "dateTimeFormats")] pub (crate) datetime_formats : DateTimeFormats , # [serde (rename = "dateTimeFormats-atTime")] pub (crate) datetime_formats_at_time : DateTimeFormatsVariant , }
};
}
