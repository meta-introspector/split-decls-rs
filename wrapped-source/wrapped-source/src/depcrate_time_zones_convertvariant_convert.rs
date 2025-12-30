// Generated macro for variant_convert (function)
macro_rules! Depcrate_time_zones_convertvariant_convert {
() => {
// Module: crate::time_zones::convert
// Provides: {"variant_convert"}
// Dependencies: {}
fn variant_convert (zone_format : & ZoneFormat) -> impl Iterator < Item = (TimeZoneVariant , & str) > { zone_format . 0 . iter () . filter (| & (variant , _) | variant != "generic") . flat_map (move | (variant , value) | { Some ((match variant . as_str () { "standard" => TimeZoneVariant :: Standard , "daylight" => TimeZoneVariant :: Daylight , _ => return None , } , value . as_str () ,)) }) }
};
}
