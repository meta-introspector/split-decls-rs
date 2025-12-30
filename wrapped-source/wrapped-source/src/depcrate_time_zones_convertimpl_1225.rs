// Generated macro for impl_1225 (impl)
macro_rules! Depcrate_time_zones_convertimpl_1225 {
() => {
// Module: crate::time_zones::convert
// Provides: {"impl_1225"}
// Dependencies: {}
impl DataProvider < TimezoneNamesGenericLongV1 > for SourceDataProvider { fn load (& self , req : DataRequest ,) -> Result < DataResponse < TimezoneNamesGenericLongV1 > , DataError > { self . check_req :: < TimezoneNamesGenericLongV1 > (req) ? ; let time_zone_names_resource = & self . cldr () ? . dates ("gregorian") . read_and_parse :: < cldr_serde :: time_zones :: time_zone_names :: Resource > (req . id . locale , "timeZoneNames.json" ,) ? . main . value . dates . time_zone_names ; let bcp47_tzid_data = self . iana_to_bcp47_map () ? ; let metazones = self . metazones () ? ; let locations = self . calculate_locations (req . id . locale) ? . 0 ; let defaults = iter_mz_defaults (time_zone_names_resource , & metazones . ids , true) . filter_map (| (mz , zf) | { let v = zf . 0 . get ("generic") ? . as_str () ; let tzs = metazones . reverse . get (& (mz , MzMembership :: Any)) ? ; let same_as_location = tzs . iter () . all (| tz | { let Some (location) = locations . get (tz) else { return false ; } ; writeable :: cmp_utf8 (& time_zone_names_resource . region_format . interpolate ([location]) , v . as_bytes () ,) == Ordering :: Equal }) ; if same_as_location { None } else { Some ((mz , v)) } }) . collect () ; let overrides = iter_mz_overrides (time_zone_names_resource , bcp47_tzid_data , true) . filter_map (| (tz , zf) | Some ((tz , zf . 0 . get ("generic") ? . as_str ()))) . collect () ; Ok (DataResponse { metadata : DataResponseMetadata :: default () . with_checksum (metazones . checksum) , payload : DataPayload :: from_owned (MetazoneGenericNames { defaults , overrides , }) , }) } }
};
}
