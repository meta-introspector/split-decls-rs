// Generated macro for impl_634 (impl)
macro_rules! Depcrate_io_uringimpl_634 {
() => {
// Module: crate::io_uring
// Provides: {"impl_634"}
// Dependencies: {}
impl io_uring_user_data { # [doc = " Create a zero-initialized `Self`."] pub const fn zeroed () -> Self { Self { u64_ : 0 } } # [doc = " Return the `u64` value."] # [inline] pub const fn u64_ (self) -> u64 { unsafe { self . u64_ } } # [doc = " Create a `Self` from a `u64` value."] # [inline] pub const fn from_u64 (u64_ : u64) -> Self { Self { u64_ } } # [doc = " Return the `ptr` pointer value."] # [inline] pub const fn ptr (self) -> * mut c_void { unsafe { self . ptr } . ptr } # [doc = " Create a `Self` from a pointer value."] # [inline] pub const fn from_ptr (ptr : * mut c_void) -> Self { Self { ptr : io_uring_ptr :: new (ptr) , } } }
};
}
