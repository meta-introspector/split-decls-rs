// Generated macro for impl_178 (impl)
macro_rules! Depcrate_deprecatedimpl_178 {
() => {
// Module: crate::deprecated
// Provides: {"impl_178"}
// Dependencies: {}
impl < B , T > Ref < B , T > where B : SplitByteSlice , T : Unaligned + KnownLayout + Immutable + ? Sized , { # [deprecated (since = "0.8.0" , note = "use `Ref::from_prefix`; for `T: Unaligned`, the returned `CastError` implements `Into<SizeError>`")] # [doc (hidden)] # [must_use = "has no side effects"] # [inline (always)] pub fn new_unaligned_from_prefix (bytes : B) -> Option < (Ref < B , T > , B) > { Self :: from_prefix (bytes) . ok () } }
};
}
