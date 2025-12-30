// Generated macro for impl_623 (impl)
macro_rules! Depcrate_io_uringimpl_623 {
() => {
// Module: crate::io_uring
// Provides: {"impl_623"}
// Dependencies: {}
impl io_uring_ptr { # [doc = " Construct a null `io_uring_ptr`."] # [inline] pub const fn null () -> Self { Self :: new (null_mut ()) } # [doc = " Construct a new `io_uring_ptr`."] # [inline] pub const fn new (ptr : * mut c_void) -> Self { Self { ptr , # [cfg (target_pointer_width = "16")] __pad16 : 0 , # [cfg (any (target_pointer_width = "16" , target_pointer_width = "32"))] __pad32 : 0 , } } }
};
}
