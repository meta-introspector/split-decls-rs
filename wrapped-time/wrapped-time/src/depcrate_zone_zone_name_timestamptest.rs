// Generated macro for test (module)
macro_rules! Depcrate_zone_zone_name_timestamptest {
() => {
// Module: crate::zone::zone_name_timestamp
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_packing () { # [derive (Debug)] struct TestCase { input : & 'static str , output : & 'static str , } for test_case in [TestCase { input : "1970-01-01T00:00Z" , output : "1970-01-01T00:00Z" , } , TestCase { input : "1970-01-01T00:01Z" , output : "1970-01-01T00:00Z" , } , TestCase { input : "1970-01-01T00:15Z" , output : "1970-01-01T00:15Z" , } , TestCase { input : "1970-01-01T00:29Z" , output : "1970-01-01T00:15Z" , } , TestCase { input : "1969-12-31T23:59Z" , output : "1970-01-01T00:00Z" , } , TestCase { input : "1969-12-31T12:00Z" , output : "1970-01-01T00:00Z" , } , TestCase { input : "1900-07-15T12:34Z" , output : "1970-01-01T00:00Z" , } , TestCase { input : "2448-06-25T15:45Z" , output : "2448-06-25T15:45Z" , } , TestCase { input : "2448-06-25T16:00Z" , output : "2448-06-25T15:45Z" , } , TestCase { input : "2448-06-26T00:00Z" , output : "2448-06-25T15:45Z" , } , TestCase { input : "2500-01-01T00:00Z" , output : "2448-06-25T15:45Z" , } , TestCase { input : "2025-10-10T10:15+02" , output : "2025-10-10T08:15Z" , } , TestCase { input : "2025-04-30T15:18:25Z" , output : "2025-04-30T15:15Z" , } ,] { let znt = ZoneNameTimestamp :: from_zoned_date_time_iso (ZonedDateTime :: try_offset_only_from_str (test_case . input , Iso) . unwrap () ,) ; let actual = znt . to_zoned_date_time_iso () ; assert_eq ! (ZonedDateTime :: try_offset_only_from_str (test_case . output , Iso) . unwrap () , actual , "{test_case:?}") ; } } }
};
}
