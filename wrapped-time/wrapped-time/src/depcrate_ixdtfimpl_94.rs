// Generated macro for impl_94 (impl)
macro_rules! Depcrate_ixdtfimpl_94 {
() => {
// Module: crate::ixdtf
// Provides: {"impl_94"}
// Dependencies: {}
impl UtcOffset { fn try_from_utc_offset_record (record : UtcOffsetRecord) -> Result < Self , ParseError > { let hour_seconds = i32 :: from (record . hour ()) * 3600 ; let minute_seconds = i32 :: from (record . minute ()) * 60 ; Self :: try_from_seconds (i32 :: from (record . sign () as i8) * (hour_seconds + minute_seconds + i32 :: from (record . second () . unwrap_or (0))) ,) . map_err (Into :: into) } }
};
}
