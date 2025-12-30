// Generated macro for impl_179 (impl)
macro_rules! Depcrate_deprecatedimpl_179 {
() => {
// Module: crate::deprecated
// Provides: {"impl_179"}
// Dependencies: {}
impl < B , T > Ref < B , T > where B : SplitByteSlice , T : Unaligned + KnownLayout + Immutable + ? Sized , { # [deprecated (since = "0.8.0" , note = "use `Ref::from_suffix`; for `T: Unaligned`, the returned `CastError` implements `Into<SizeError>`")] # [doc (hidden)] # [must_use = "has no side effects"] # [inline (always)] pub fn new_unaligned_from_suffix (bytes : B) -> Option < (B , Ref < B , T >) > { Self :: from_suffix (bytes) . ok () } }
};
}
