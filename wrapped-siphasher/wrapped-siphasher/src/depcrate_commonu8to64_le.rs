// Generated macro for u8to64_le (function)
macro_rules! Depcrate_commonu8to64_le {
() => {
// Module: crate::common
// Provides: {"u8to64_le"}
// Dependencies: {}
# [doc = " Loads a u64 using up to 7 bytes of a byte slice."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that `start + len <= buf.len()` and `len < 8`."] # [inline] pub unsafe fn u8to64_le (buf : & [u8] , start : usize , len : usize) -> u64 { debug_assert ! (len < 8) ; let mut i = 0 ; let mut out = 0 ; if i + 3 < len { out = load_int_le ! (buf , start + i , u32) as u64 ; i += 4 ; } if i + 1 < len { out |= (load_int_le ! (buf , start + i , u16) as u64) << (i * 8) ; i += 2 ; } if i < len { out |= (* buf . get_unchecked (start + i) as u64) << (i * 8) ; i += 1 ; } debug_assert_eq ! (i , len) ; out }
};
}
