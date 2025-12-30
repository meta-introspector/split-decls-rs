// Generated macro for test_legacy_offsets_data (function)
macro_rules! Depcrate_zone_offsettest_legacy_offsets_data {
() => {
// Module: crate::zone::offset
// Provides: {"test_legacy_offsets_data"}
// Dependencies: {}
# [test] # [allow (deprecated)] pub fn test_legacy_offsets_data () { use crate :: ZonedDateTime ; use icu_locale_core :: subtags :: subtag ; use icu_provider_blob :: BlobDataProvider ; let c = VariantOffsetsCalculator :: try_new_with_buffer_provider (& BlobDataProvider :: try_new_from_static_blob (include_bytes ! ("../../tests/data/offset_periods_old.blob") ,) . unwrap () ,) . unwrap () ; let tz = TimeZone (subtag ! ("aqcas")) ; for timestamp in ["1970-01-01 00:00Z" , "2009-10-17 18:00Z" , "2010-03-04 15:00Z" , "2011-10-27 18:00Z" , "2012-02-21 17:00Z" , "2016-10-21 16:00Z" , "2018-03-10 17:00Z" , "2018-10-06 20:00Z" , "2019-03-16 16:00Z" , "2019-10-03 19:00Z" , "2020-03-07 16:00Z" , "2021-03-13 13:00Z" , "2022-03-12 13:00Z" , "2023-03-08 16:00Z" ,] { let t = ZoneNameTimestamp :: from_zoned_date_time_iso (ZonedDateTime :: try_offset_only_from_str (timestamp , icu_calendar :: Iso) . unwrap () ,) ; assert_eq ! (c . as_borrowed () . compute_offsets_from_time_zone_and_name_timestamp (tz , t) , VariantOffsetsCalculator :: new () . compute_offsets_from_time_zone_and_name_timestamp (tz , t) , "{timestamp:?}" ,) ; } }
};
}
