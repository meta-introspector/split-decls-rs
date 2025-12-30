// Generated macro for test (module)
macro_rules! Depcrate_ixdtftest {
() => {
// Module: crate::ixdtf
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: TimeZone ; # [test] fn max_possible_rfc_9557_utc_offset () { assert_eq ! (ZonedDateTime :: try_offset_only_from_str ("2024-08-08T12:08:19+23:59:59.999999999" , Iso) . unwrap_err () , ParseError :: InvalidOffsetError) ; } # [test] fn zone_calculations () { ZonedDateTime :: try_offset_only_from_str ("2024-08-08T12:08:19Z" , Iso) . unwrap () ; assert_eq ! (ZonedDateTime :: try_offset_only_from_str ("2024-08-08T12:08:19Z[+08:00]" , Iso) . unwrap_err () , ParseError :: RequiresCalculation) ; assert_eq ! (ZonedDateTime :: try_offset_only_from_str ("2024-08-08T12:08:19Z[Europe/Zurich]" , Iso) . unwrap_err () , ParseError :: MismatchedTimeZoneFields) ; } # [test] fn future_zone () { let result = ZonedDateTime :: try_location_only_from_str ("2024-08-08T12:08:19[Future/Zone]" , Iso , IanaParserBorrowed :: new () ,) . unwrap () ; assert_eq ! (result . zone . id () , TimeZone :: UNKNOWN) ; assert_eq ! (result . zone . offset () , None) ; } # [test] fn lax () { ZonedDateTime :: try_location_only_from_str ("2024-10-18T15:44[America/Los_Angeles]" , icu_calendar :: cal :: Gregorian , IanaParserBorrowed :: new () ,) . unwrap () ; } }
};
}
