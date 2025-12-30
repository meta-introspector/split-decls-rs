// Generated macro for variant_fallback (function)
macro_rules! Depcrate_time_zones_convertvariant_fallback {
() => {
// Module: crate::time_zones::convert
// Provides: {"variant_fallback"}
// Dependencies: {}
# [doc = " Performs part 1 of type fallback as specified in the UTS-35 spec for TimeZone Goals:"] # [doc = " https://unicode.org/reports/tr35/tr35-dates.html#Time_Zone_Goals"] # [doc = ""] # [doc = " Part 2 of type fallback requires access to the IANA TimeZone Database"] # [doc = " as well as a specific datetime context, so it is not relevant to DataProvider."] fn variant_fallback (zone_format : & ZoneFormat) -> Option < & str > { zone_format . 0 . get ("generic") . or_else (| | zone_format . 0 . get ("standard")) . map (| s | s . as_str ()) }
};
}
