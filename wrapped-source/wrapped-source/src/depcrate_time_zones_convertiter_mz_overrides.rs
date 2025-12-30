// Generated macro for iter_mz_overrides (function)
macro_rules! Depcrate_time_zones_convertiter_mz_overrides {
() => {
// Module: crate::time_zones::convert
// Provides: {"iter_mz_overrides"}
// Dependencies: {}
fn iter_mz_overrides < 'a > (time_zone_names_resource : & 'a TimeZoneNames , bcp47_tzid_data : & 'a BTreeMap < String , TimeZone > , is_long : bool ,) -> impl Iterator < Item = (TimeZone , & 'a ZoneFormat) > { time_zone_names_resource . zone . 0 . iter () . flat_map (move | (key , region) | { region . 0 . iter () . flat_map (move | (inner_key , place_or_region) | { let iana = format ! ("{key}/{inner_key}") ; let Some (& tz) = bcp47_tzid_data . get (& iana) else { return Default :: default () ; } ; match place_or_region { LocationOrSubRegion :: Location (place) => place . long_short (is_long) . map (| zf | (tz , zf)) . into_iter () . collect :: < Vec < _ > > () , LocationOrSubRegion :: SubRegion (region) => region . iter () . filter_map (| (inner_key , place) | { let iana = format ! ("{iana}/{inner_key}") ; let Some (& tz) = bcp47_tzid_data . get (& iana) else { return Default :: default () ; } ; place . long_short (is_long) . map (| zf | (tz , zf)) }) . collect () , } }) }) }
};
}
