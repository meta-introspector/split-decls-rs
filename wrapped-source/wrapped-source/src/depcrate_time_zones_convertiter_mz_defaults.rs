// Generated macro for iter_mz_defaults (function)
macro_rules! Depcrate_time_zones_convertiter_mz_defaults {
() => {
// Module: crate::time_zones::convert
// Provides: {"iter_mz_defaults"}
// Dependencies: {}
fn iter_mz_defaults < 'a > (time_zone_names_resource : & 'a TimeZoneNames , meta_zone_id_data : & 'a BTreeMap < String , MetazoneId > , is_long : bool ,) -> impl Iterator < Item = (MetazoneId , & 'a ZoneFormat) > + 'a { time_zone_names_resource . metazone . as_ref () . map (| m | & m . 0) . unwrap_or ({ static EMPTY : BTreeMap < String , Metazone > = BTreeMap :: new () ; & EMPTY }) . iter () . filter_map (move | (key , metazone) | { Some ((* meta_zone_id_data . get (key) ? , metazone . long_short (is_long) ?)) }) }
};
}
