// Generated macro for impl_686 (impl)
macro_rules! Depcrate_datetime_week_dataimpl_686 {
() => {
// Module: crate::datetime::week_data
// Provides: {"impl_686"}
// Dependencies: {}
impl DataProvider < CalendarWeekV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < CalendarWeekV1 > , DataError > { self . check_req :: < CalendarWeekV1 > (req) ? ; let territory = req . id . locale . region . map (Territory :: Region) . unwrap_or_else (| | DEFAULT_TERRITORY . clone ()) ; let week_data : & cldr_serde :: week_data :: Resource = self . cldr () ? . core () . read_and_parse ("supplemental/weekData.json") ? ; let week_data = & week_data . supplemental . week_data ; let first_weekday : icu :: calendar :: types :: Weekday = week_data . first_day . get (& territory) . or_else (| | week_data . first_day . get (& DEFAULT_TERRITORY)) . ok_or (DataError :: custom ("Missing default entry for firstDay in weekData.json" ,)) ? . into () ; let weekend = { let weekend_start = week_data . weekend_start . get (& territory) . or_else (| | week_data . weekend_start . get (& DEFAULT_TERRITORY)) . ok_or (DataError :: custom ("Missing default entry for weekendStart in weekData.json" ,)) ? ; let weekend_end = week_data . weekend_end . get (& territory) . or_else (| | week_data . weekend_end . get (& DEFAULT_TERRITORY)) . ok_or (DataError :: custom ("Missing default entry for weekendEnd in weekData.json" ,)) ? ; WeekdaySet :: new (& [weekend_start . into () , weekend_end . into ()]) } ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (WeekData { first_weekday , weekend , }) , }) } }
};
}
