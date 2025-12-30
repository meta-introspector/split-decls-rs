// Generated macro for TimeZoneAndCanonical (struct)
macro_rules! Depcrate_zone_ianaTimeZoneAndCanonical {
() => {
// Module: crate::zone::iana
// Provides: {"TimeZoneAndCanonical"}
// Dependencies: {}
# [doc = " Return value of [`IanaParserBorrowed::iter`]."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [non_exhaustive] pub struct TimeZoneAndCanonical < 'a > { # [doc = " The parsed [`TimeZone`]"] pub time_zone : TimeZone , # [doc = " The canonical IANA ID"] pub canonical : & 'a str , }
};
}
