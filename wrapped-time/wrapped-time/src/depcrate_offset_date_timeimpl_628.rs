// Generated macro for impl_628 (impl)
macro_rules! Depcrate_offset_date_timeimpl_628 {
() => {
// Module: crate::offset_date_time
// Provides: {"impl_628"}
// Dependencies: {}
impl PartialEq for OffsetDateTime { # [inline] fn eq (& self , other : & Self) -> bool { raw_to_bits ((self . year () , self . ordinal () , self . time ())) == raw_to_bits (other . to_offset_raw (self . offset ())) } }
};
}
