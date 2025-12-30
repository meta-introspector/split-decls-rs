// Generated macro for TimeZoneVariantInfo (struct)
macro_rules! Depcrate_data_posixTimeZoneVariantInfo {
() => {
// Module: crate::data::posix
// Provides: {"TimeZoneVariantInfo"}
// Dependencies: {}
# [doc = " A struct to hold a time-zone variant name and its offset."] # [doc = " The offset is how many hours must be added to the time to reach UTC."] # [derive (Debug , Clone , PartialEq , Eq)] pub struct TimeZoneVariantInfo { # [doc = " The name of the time-zone variant."] pub name : String , # [doc = " The offset time in seconds that must be added to reach UTC."] pub offset : Seconds , }
};
}
