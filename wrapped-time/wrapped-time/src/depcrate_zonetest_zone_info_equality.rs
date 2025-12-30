// Generated macro for test_zone_info_equality (function)
macro_rules! Depcrate_zonetest_zone_info_equality {
() => {
// Module: crate::zone
// Provides: {"test_zone_info_equality"}
// Dependencies: {}
# [test] fn test_zone_info_equality () { assert_eq ! (IanaParser :: new () . parse ("Etc/GMT-8") . with_offset (None) , TimeZone :: UNKNOWN . with_offset (Some (UtcOffset :: from_seconds_unchecked (8 * 60 * 60)))) ; assert_eq ! (IanaParser :: new () . parse ("Etc/UTC") . with_offset (None) , TimeZoneInfo :: utc ()) ; assert_eq ! (IanaParser :: new () . parse ("Etc/GMT") . with_offset (None) , IanaParser :: new () . parse ("Etc/GMT") . with_offset (Some (UtcOffset :: zero ()))) ; assert_eq ! (IanaParser :: new () . parse ("Etc/GMT-8") . with_offset (Some (UtcOffset :: from_seconds_unchecked (123))) , TimeZoneInfo :: unknown ()) ; assert_eq ! (IanaParser :: new () . parse ("Etc/UTC") . with_offset (Some (UtcOffset :: from_seconds_unchecked (123))) , TimeZoneInfo :: unknown () ,) ; assert_eq ! (IanaParser :: new () . parse ("Etc/GMT") . with_offset (Some (UtcOffset :: from_seconds_unchecked (123))) , TimeZoneInfo :: unknown ()) ; }
};
}
