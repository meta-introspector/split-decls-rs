// Generated macro for TimeZoneAndCanonicalAndNormalized (struct)
macro_rules! Depcrate_zone_ianaTimeZoneAndCanonicalAndNormalized {
() => {
// Module: crate::zone::iana
// Provides: {"TimeZoneAndCanonicalAndNormalized"}
// Dependencies: {}
# [doc = " Return value of [`IanaParserExtendedBorrowed::parse`], [`IanaParserExtendedBorrowed::iter`]."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [non_exhaustive] pub struct TimeZoneAndCanonicalAndNormalized < 'a > { # [doc = " The parsed [`TimeZone`]"] pub time_zone : TimeZone , # [doc = " The canonical IANA ID"] pub canonical : & 'a str , # [doc = " The normalized IANA ID"] pub normalized : & 'a str , }
};
}
