// Generated macro for impl_181 (impl)
macro_rules! Depcrate_deprecatedimpl_181 {
() => {
// Module: crate::deprecated
// Provides: {"impl_181"}
// Dependencies: {}
impl < B , T > Ref < B , [T] > where B : ByteSlice , T : Unaligned + Immutable , { # [deprecated (since = "0.8.0" , note = "`Ref::from_bytes` now supports slices; for `T: Unaligned`, the returned `CastError` implements `Into<SizeError>`")] # [doc (hidden)] # [inline (always)] pub fn new_slice_unaligned (bytes : B) -> Option < Ref < B , [T] > > { Ref :: from_bytes (bytes) . ok () } }
};
}
