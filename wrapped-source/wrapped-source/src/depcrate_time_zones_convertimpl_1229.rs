// Generated macro for impl_1229 (impl)
macro_rules! Depcrate_time_zones_convertimpl_1229 {
() => {
// Module: crate::time_zones::convert
// Provides: {"impl_1229"}
// Dependencies: {}
impl DataProvider < TimezoneNamesSpecificShortV1 > for SourceDataProvider { fn load (& self , req : DataRequest ,) -> Result < DataResponse < TimezoneNamesSpecificShortV1 > , DataError > { self . check_req :: < TimezoneNamesSpecificShortV1 > (req) ? ; let time_zone_names_resource = & self . cldr () ? . dates ("gregorian") . read_and_parse :: < cldr_serde :: time_zones :: time_zone_names :: Resource > (req . id . locale , "timeZoneNames.json" ,) ? . main . value . dates . time_zone_names ; let bcp47_tzid_data = self . iana_to_bcp47_map () ? ; let metazones = self . metazones () ? ; let defaults = iter_mz_defaults (time_zone_names_resource , & metazones . ids , false) . flat_map (| (mz , zf) | variant_convert (zf) . map (move | (zv , v) | ((mz , zv) , v))) . collect () ; let overrides = iter_mz_overrides (time_zone_names_resource , bcp47_tzid_data , false) . flat_map (| (tz , zf) | variant_convert (zf) . map (move | (zv , v) | ((tz , zv) , v))) . collect () ; Ok (DataResponse { metadata : DataResponseMetadata :: default () . with_checksum (metazones . checksum) , payload : DataPayload :: from_owned (MetazoneSpecificNames { defaults , overrides , use_standard : Default :: default () , }) , }) } }
};
}
