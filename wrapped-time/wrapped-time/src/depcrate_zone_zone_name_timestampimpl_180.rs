// Generated macro for impl_180 (impl)
macro_rules! Depcrate_zone_zone_name_timestampimpl_180 {
() => {
// Module: crate::zone::zone_name_timestamp
// Provides: {"impl_180"}
// Dependencies: {}
# [cfg (feature = "serde")] impl serde :: Serialize for ZoneNameTimestamp { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { # [cfg (feature = "alloc")] if serializer . is_human_readable () { let date_time = self . to_zoned_date_time_iso () ; let year = date_time . date . era_year () . year ; let month = date_time . date . month () . number () ; let day = date_time . date . day_of_month () . 0 ; let hour = date_time . time . hour . number () ; let minute = date_time . time . minute . number () ; let second = date_time . time . second . number () ; let mut s = alloc :: format ! ("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}") ; if second != 0 { use alloc :: fmt :: Write ; let _infallible = write ! (& mut s , ":{second:02}") ; } return serializer . serialize_str (& s) ; } serializer . serialize_u32 (self . 0) } }
};
}
