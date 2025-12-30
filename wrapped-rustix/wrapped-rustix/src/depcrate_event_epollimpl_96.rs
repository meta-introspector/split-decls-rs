// Generated macro for impl_96 (impl)
macro_rules! Depcrate_event_epollimpl_96 {
() => {
// Module: crate::event::epoll
// Provides: {"impl_96"}
// Dependencies: {}
impl EventData { # [doc = " Construct a new value containing a `u64`."] # [inline] pub const fn new_u64 (value : u64) -> Self { Self { as_u64 : value } } # [doc = " Construct a new value containing a `*mut c_void`."] # [inline] pub const fn new_ptr (value : * mut c_void) -> Self { Self { sixty_four_bit_pointer : SixtyFourBitPointer { pointer : value , # [cfg (target_pointer_width = "32")] _padding : 0 , } , } } # [doc = " Return the value as a `u64`."] # [doc = ""] # [doc = " If the stored value was a pointer, the pointer is zero-extended to a"] # [doc = " `u64`."] # [inline] pub fn u64 (self) -> u64 { unsafe { self . as_u64 } } # [doc = " Return the value as a `*mut c_void`."] # [doc = ""] # [doc = " If the stored value was a `u64`, the least-significant bits of the"] # [doc = " `u64` are returned as a pointer value."] # [inline] pub fn ptr (self) -> * mut c_void { unsafe { self . sixty_four_bit_pointer . pointer } } }
};
}
