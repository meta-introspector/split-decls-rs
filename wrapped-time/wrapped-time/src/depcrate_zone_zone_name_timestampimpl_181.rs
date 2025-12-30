// Generated macro for impl_181 (impl)
macro_rules! Depcrate_zone_zone_name_timestampimpl_181 {
() => {
// Module: crate::zone::zone_name_timestamp
// Provides: {"impl_181"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde :: Deserialize < 'de > for ZoneNameTimestamp { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { # [cfg (feature = "alloc")] if deserializer . is_human_readable () { use serde :: de :: Error ; let e0 = D :: Error :: custom ("invalid") ; let e1 = | _ | D :: Error :: custom ("invalid") ; let e2 = | _ | D :: Error :: custom ("invalid") ; let e3 = | _ | D :: Error :: custom ("invalid") ; let parts = alloc :: borrow :: Cow :: < 'de , str > :: deserialize (deserializer) ? ; if parts . len () != 16 { return Err (e0) ; } let year = parts [0 .. 4] . parse :: < i32 > () . map_err (e1) ? ; let month = parts [5 .. 7] . parse :: < u8 > () . map_err (e1) ? ; let day = parts [8 .. 10] . parse :: < u8 > () . map_err (e1) ? ; let hour = parts [11 .. 13] . parse :: < u8 > () . map_err (e1) ? ; let minute = parts [14 .. 16] . parse :: < u8 > () . map_err (e1) ? ; return Ok (Self :: from_zoned_date_time_iso (ZonedDateTime { date : icu_calendar :: Date :: try_new_iso (year , month , day) . map_err (e2) ? , time : crate :: Time :: try_new (hour , minute , 0 , 0) . map_err (e3) ? , zone : UtcOffset :: zero () , })) ; } u32 :: deserialize (deserializer) . map (Self) } }
};
}
