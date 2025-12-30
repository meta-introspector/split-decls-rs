// Generated macro for unsafe_slice (function)
macro_rules! Depcrate_utilunsafe_slice {
() => {
// Module: crate::util
// Provides: {"unsafe_slice"}
// Dependencies: {}
# [inline (always)] pub unsafe fn unsafe_slice < 'a > (buf : & 'a [u8] , start : usize , new_len : usize) -> & 'a [u8] { debug_assert ! (start <= buf . len ()) ; debug_assert ! (new_len <= (buf . len () - start)) ; slice :: from_raw_parts (buf . as_ptr () . offset (start as isize) , new_len) }
};
}
