// Generated macro for TimeZoneNames (struct)
macro_rules! Depcrate_cldr_serde_time_zones_time_zone_namesTimeZoneNames {
() => {
// Module: crate::cldr_serde::time_zones::time_zone_names
// Provides: {"TimeZoneNames"}
// Dependencies: {}
# [derive (PartialEq , Debug , Default , Clone)] pub (crate) struct TimeZoneNames { pub (crate) hour_format : String , pub (crate) gmt_format : PatternString < SinglePlaceholder > , pub (crate) gmt_zero_format : String , pub (crate) gmt_unknown_format : String , pub (crate) region_format : PatternString < SinglePlaceholder > , pub (crate) region_format_dt : PatternString < SinglePlaceholder > , pub (crate) region_format_st : PatternString < SinglePlaceholder > , pub (crate) fallback_format : PatternString < DoublePlaceholder > , pub (crate) zone : Zones , pub (crate) metazone : Option < Metazones > , }
};
}
