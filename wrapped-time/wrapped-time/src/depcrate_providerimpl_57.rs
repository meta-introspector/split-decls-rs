// Generated macro for impl_57 (impl)
macro_rules! Depcrate_providerimpl_57 {
() => {
// Module: crate::provider
// Provides: {"impl_57"}
// Dependencies: {}
impl AsULE for VariantOffsets { type ULE = [i8 ; 2] ; fn from_unaligned ([std , dst] : Self :: ULE) -> Self { fn decode (encoded : i8) -> i32 { encoded as i32 * SECONDS_TO_EIGHTS_OF_HOURS + match encoded % 8 { 1 | 5 => 150 , - 1 | - 5 => - 150 , 3 | 7 => - 150 , - 3 | - 7 => 150 , _ => 0 , } } Self { standard : UtcOffset :: from_seconds_unchecked (decode (std)) , daylight : (dst != 0) . then (| | UtcOffset :: from_seconds_unchecked (decode (std + dst))) , } } fn to_unaligned (self) -> Self :: ULE { fn encode (offset : i32) -> i8 { debug_assert_eq ! (offset . abs () % 60 , 0) ; let scaled = match offset . abs () / 60 % 60 { 0 | 15 | 30 | 45 => offset / SECONDS_TO_EIGHTS_OF_HOURS , 10 | 40 => { offset / SECONDS_TO_EIGHTS_OF_HOURS } 20 | 50 => { offset / SECONDS_TO_EIGHTS_OF_HOURS + offset . signum () } _ => { debug_assert ! (false , "{offset:?}") ; offset / SECONDS_TO_EIGHTS_OF_HOURS } } ; debug_assert ! (i8 :: MIN as i32 <= scaled && scaled <= i8 :: MAX as i32) ; scaled as i8 } [encode (self . standard . to_seconds ()) , self . daylight . map (| d | encode (d . to_seconds () - self . standard . to_seconds ())) . unwrap_or_default () ,] } }
};
}
