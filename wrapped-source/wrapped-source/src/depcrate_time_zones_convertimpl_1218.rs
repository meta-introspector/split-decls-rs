// Generated macro for impl_1218 (impl)
macro_rules! Depcrate_time_zones_convertimpl_1218 {
() => {
// Module: crate::time_zones::convert
// Provides: {"impl_1218"}
// Dependencies: {}
impl DataProvider < TimezoneNamesEssentialsV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < TimezoneNamesEssentialsV1 > , DataError > { self . check_req :: < TimezoneNamesEssentialsV1 > (req) ? ; let time_zone_names = & self . cldr () ? . dates ("gregorian") . read_and_parse :: < cldr_serde :: time_zones :: time_zone_names :: Resource > (req . id . locale , "timeZoneNames.json" ,) ? . main . value . dates . time_zone_names ; let offset_separator = self . load_duration_parts_internal (req) ? . 2 . to_owned () . into () ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (TimeZoneEssentials { offset_separator , offset_pattern : Cow :: Owned (time_zone_names . gmt_format . 0 . clone ()) , offset_zero : time_zone_names . gmt_zero_format . clone () . into () , offset_unknown : time_zone_names . gmt_unknown_format . clone () . into () , }) , }) } }
};
}
