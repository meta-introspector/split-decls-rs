// Generated macro for impl_31 (impl)
macro_rules! Depcrateimpl_31 {
() => {
// Module: crate
// Provides: {"impl_31"}
// Dependencies: {}
impl < 'a > Iterator for Utf8Chars < 'a > { type Item = char ; # [inline] fn next (& mut self) -> Option < char > { # [allow (clippy :: never_loop)] loop { if self . remaining . len () < 4 { break ; } let first = self . remaining [0] ; if first < 0x80 { self . remaining = & self . remaining [1 ..] ; return Some (char :: from (first)) ; } let second = self . remaining [1] ; if in_inclusive_range8 (first , 0xC2 , 0xDF) { if ! in_inclusive_range8 (second , 0x80 , 0xBF) { break ; } let point = ((u32 :: from (first) & 0x1F) << 6) | (u32 :: from (second) & 0x3F) ; self . remaining = & self . remaining [2 ..] ; return Some (unsafe { char :: from_u32_unchecked (point) }) ; } let third = self . remaining [2] ; if first < 0xF0 { if ((UTF8_DATA . table [usize :: from (second)] & UTF8_DATA . table [usize :: from (first) + 0x80]) | (third >> 6)) != 2 { break ; } let point = ((u32 :: from (first) & 0xF) << 12) | ((u32 :: from (second) & 0x3F) << 6) | (u32 :: from (third) & 0x3F) ; self . remaining = & self . remaining [3 ..] ; return Some (unsafe { char :: from_u32_unchecked (point) }) ; } let fourth = self . remaining [3] ; if (u16 :: from (UTF8_DATA . table [usize :: from (second)] & UTF8_DATA . table [usize :: from (first) + 0x80] ,) | u16 :: from (third >> 6) | (u16 :: from (fourth & 0xC0) << 2)) != 0x202 { break ; } let point = ((u32 :: from (first) & 0x7) << 18) | ((u32 :: from (second) & 0x3F) << 12) | ((u32 :: from (third) & 0x3F) << 6) | (u32 :: from (fourth) & 0x3F) ; self . remaining = & self . remaining [4 ..] ; return Some (unsafe { char :: from_u32_unchecked (point) }) ; } self . next_fallback () } }
};
}
