// Generated macro for impl_630 (impl)
macro_rules! Depcrate_offset_date_timeimpl_630 {
() => {
// Module: crate::offset_date_time
// Provides: {"impl_630"}
// Dependencies: {}
impl Ord for OffsetDateTime { # [inline] fn cmp (& self , other : & Self) -> Ordering { raw_to_bits ((self . year () , self . ordinal () , self . time ())) . cmp (& raw_to_bits (other . to_offset_raw (self . offset ()))) } }
};
}
