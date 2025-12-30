// Generated macro for unsafe_slice_mut (function)
macro_rules! Depcrate_utilunsafe_slice_mut {
() => {
// Module: crate::util
// Provides: {"unsafe_slice_mut"}
// Dependencies: {}
# [inline (always)] pub unsafe fn unsafe_slice_mut < 'a > (buf : & 'a mut [u8] , start : usize , new_len : usize) -> & 'a mut [u8] { debug_assert ! (start <= buf . len ()) ; debug_assert ! (new_len <= (buf . len () - start)) ; slice :: from_raw_parts_mut (buf . as_mut_ptr () . offset (start as isize) , new_len) }
};
}
