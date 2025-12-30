// Generated macro for impl_1221 (impl)
macro_rules! Depcrate_time_zones_convertimpl_1221 {
() => {
// Module: crate::time_zones::convert
// Provides: {"impl_1221"}
// Dependencies: {}
impl DataProvider < TimezoneNamesLocationsRootV1 > for SourceDataProvider { fn load (& self , req : DataRequest ,) -> Result < DataResponse < TimezoneNamesLocationsRootV1 > , DataError > { self . check_req :: < TimezoneNamesLocationsOverrideV1 > (req) ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (Locations { locations : self . calculate_locations (& self . dedupe_group (* req . id . locale) ?) ? . 0 . into_iter () . collect () , pattern_generic : Default :: default () , pattern_standard : Default :: default () , pattern_daylight : Default :: default () , pattern_partial_location : Default :: default () , }) , }) } }
};
}
