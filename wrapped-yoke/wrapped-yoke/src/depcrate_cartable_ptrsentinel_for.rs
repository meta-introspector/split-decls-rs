// Generated macro for sentinel_for (function)
macro_rules! Depcrate_cartable_ptrsentinel_for {
() => {
// Module: crate::cartable_ptr
// Provides: {"sentinel_for"}
// Dependencies: {}
# [inline] fn sentinel_for < T > () -> NonNull < T > { static SENTINEL : & u8 = & 0x1a ; unsafe { NonNull :: new_unchecked (SENTINEL as * const u8 as * mut T) } }
};
}
