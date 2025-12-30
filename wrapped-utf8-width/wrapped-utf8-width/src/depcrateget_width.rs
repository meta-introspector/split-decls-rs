// Generated macro for get_width (function)
macro_rules! Depcrateget_width {
() => {
// Module: crate
// Provides: {"get_width"}
// Dependencies: {}
# [doc = " Given a first byte, determine how many bytes are in this UTF-8 character. If the UTF-8 character is invalid, return `0`; otherwise, return `1` to `4`."] # [inline] pub const fn get_width (byte : u8) -> usize { if is_width_1 (byte) { 1 } else if is_width_2 (byte) { 2 } else if byte <= MAX_3 { 3 } else if byte <= MAX_4 { 4 } else { 0 } }
};
}
