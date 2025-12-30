// Generated macro for impl_176 (impl)
macro_rules! Depcrate_deprecatedimpl_176 {
() => {
// Module: crate::deprecated
// Provides: {"impl_176"}
// Dependencies: {}
impl < B , T > Ref < B , T > where B : SplitByteSlice , T : KnownLayout + Immutable + ? Sized , { # [deprecated (since = "0.8.0" , note = "renamed to `Ref::from_suffix`")] # [doc (hidden)] # [must_use = "has no side effects"] # [inline (always)] pub fn new_from_suffix (bytes : B) -> Option < (B , Ref < B , T >) > { Self :: from_suffix (bytes) . ok () } }
};
}
