// Generated macro for impl_175 (impl)
macro_rules! Depcrate_deprecatedimpl_175 {
() => {
// Module: crate::deprecated
// Provides: {"impl_175"}
// Dependencies: {}
impl < B , T > Ref < B , T > where B : SplitByteSlice , T : KnownLayout + Immutable + ? Sized , { # [deprecated (since = "0.8.0" , note = "renamed to `Ref::from_prefix`")] # [doc (hidden)] # [must_use = "has no side effects"] # [inline (always)] pub fn new_from_prefix (bytes : B) -> Option < (Ref < B , T > , B) > { Self :: from_prefix (bytes) . ok () } }
};
}
