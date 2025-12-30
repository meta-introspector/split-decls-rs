// Generated macro for impl_1220 (impl)
macro_rules! Depcrate_time_zones_convertimpl_1220 {
() => {
// Module: crate::time_zones::convert
// Provides: {"impl_1220"}
// Dependencies: {}
impl DataProvider < TimezoneNamesLocationsOverrideV1 > for SourceDataProvider { fn load (& self , req : DataRequest ,) -> Result < DataResponse < TimezoneNamesLocationsOverrideV1 > , DataError > { self . check_req :: < TimezoneNamesLocationsOverrideV1 > (req) ? ; let time_zone_names = & self . cldr () ? . dates ("gregorian") . read_and_parse :: < cldr_serde :: time_zones :: time_zone_names :: Resource > (req . id . locale , "timeZoneNames.json" ,) ? . main . value . dates . time_zone_names ; let mut locations = self . calculate_locations (req . id . locale) ? . 0 ; let base = DataProvider :: < TimezoneNamesLocationsRootV1 > :: load (& self , req) ? . payload ; locations . retain (| k , v | base . get () . locations . get (k) != Some (v)) ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (Locations { locations : locations . into_iter () . collect () , pattern_generic : Cow :: Owned (time_zone_names . region_format . 0 . clone ()) , pattern_standard : Cow :: Owned (time_zone_names . region_format_st . 0 . clone ()) , pattern_daylight : Cow :: Owned (time_zone_names . region_format_dt . 0 . clone ()) , pattern_partial_location : Cow :: Owned (time_zone_names . fallback_format . 0 . clone ()) , }) , }) } }
};
}
