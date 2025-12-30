// Generated macro for impl_691 (impl)
macro_rules! Depcrate_datetimeimpl_691 {
() => {
// Module: crate::datetime
// Provides: {"impl_691"}
// Dependencies: {}
impl SourceDataProvider { pub (crate) fn get_dates_resource (& self , locale : & DataLocale , calendar : Option < DatagenCalendar > ,) -> Result < & cldr_serde :: ca :: Dates , DataError > { let cldr_cal = calendar . map (DatagenCalendar :: cldr_name) . unwrap_or ("generic") ; Ok (self . cldr () ? . dates (cldr_cal) . read_and_parse :: < cldr_serde :: ca :: Resource > (locale , & format ! ("ca-{cldr_cal}.json")) ? . main . value . dates . calendars . get (cldr_cal) . expect ("CLDR file contains the expected calendar")) } }
};
}
