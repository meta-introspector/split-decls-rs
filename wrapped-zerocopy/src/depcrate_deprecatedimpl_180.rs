// Generated macro for impl_180 (impl)
macro_rules! Depcrate_deprecatedimpl_180 {
() => {
// Module: crate::deprecated
// Provides: {"impl_180"}
// Dependencies: {}
impl < B , T > Ref < B , [T] > where B : ByteSlice , T : Immutable , { # [deprecated (since = "0.8.0" , note = "`Ref::from_bytes` now supports slices")] # [doc (hidden)] # [inline (always)] pub fn new_slice (bytes : B) -> Option < Ref < B , [T] > > { Self :: from_bytes (bytes) . ok () } }
};
}
