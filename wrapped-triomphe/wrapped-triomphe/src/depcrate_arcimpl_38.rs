// Generated macro for impl_38 (impl)
macro_rules! Depcrate_arcimpl_38 {
() => {
// Module: crate::arc
// Provides: {"impl_38"}
// Dependencies: {}
impl < T > Arc < [T] > { # [doc = " Reconstruct the `Arc<[T]>` from a raw pointer obtained from `into_raw()`."] # [doc = ""] # [doc = " [`Arc::from_raw`] should accept unsized types, but this is not trivial to do correctly"] # [doc = " until the feature [`pointer_bytes_offsets`](https://github.com/rust-lang/rust/issues/96283)"] # [doc = " is stabilized. This is stopgap solution for slices."] # [doc = ""] # [doc = "  # Safety"] # [doc = " - The given pointer must be a valid pointer to `[T]` that came from [`Arc::into_raw`]."] # [doc = " - After `from_raw_slice`, the pointer must not be accessed."] pub unsafe fn from_raw_slice (ptr : * const [T]) -> Self { Arc :: from_raw (ptr) } }
};
}
