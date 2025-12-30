// Generated macro for raw_to_bits (function)
macro_rules! Depcrate_offset_date_timeraw_to_bits {
() => {
// Module: crate::offset_date_time
// Provides: {"raw_to_bits"}
// Dependencies: {}
# [doc = " **Note**: This value is explicitly signed, so do not cast this to or treat this as an"] # [doc = " unsigned integer. Doing so will lead to incorrect results for values with differing"] # [doc = " signs."] # [inline] const fn raw_to_bits ((year , ordinal , time) : (i32 , u16 , Time)) -> i128 { ((year as i128) << 74) | ((ordinal as i128) << 64) | (time . as_u64 () as i128) }
};
}
