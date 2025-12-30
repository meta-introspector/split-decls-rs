// Generated macro for format_hyphenated (function)
macro_rules! Depcrate_fmtformat_hyphenated {
() => {
// Module: crate::fmt
// Provides: {"format_hyphenated"}
// Dependencies: {}
# [inline] const fn format_hyphenated (src : & [u8 ; 16] , upper : bool) -> [u8 ; 36] { let lut = if upper { & UPPER } else { & LOWER } ; let groups = [(0 , 8) , (9 , 13) , (14 , 18) , (19 , 23) , (24 , 36)] ; let mut dst = [0 ; 36] ; let mut group_idx = 0 ; let mut i = 0 ; while group_idx < 5 { let (start , end) = groups [group_idx] ; let mut j = start ; while j < end { let x = src [i] ; i += 1 ; dst [j] = lut [(x >> 4) as usize] ; dst [j + 1] = lut [(x & 0x0f) as usize] ; j += 2 ; } if group_idx < 4 { dst [end] = b'-' ; } group_idx += 1 ; } dst }
};
}
