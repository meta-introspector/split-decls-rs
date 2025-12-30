// Generated macro for get_width_assume_valid (function)
macro_rules! Depcrateget_width_assume_valid {
() => {
// Module: crate
// Provides: {"get_width_assume_valid"}
// Dependencies: {}
# [doc = " *Assuming the input first byte is from a valid UTF-8 character*, determine how many bytes are in this UTF-8 character. It returns `1` to `4`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " You must ensure that the input byte is a valid UTF-8 first byte on your own."] # [inline] pub const unsafe fn get_width_assume_valid (byte : u8) -> usize { if byte <= MAX_1 { 1 } else if byte <= MAX_2 { 2 } else if byte <= MAX_3 { 3 } else { 4 } }
};
}
