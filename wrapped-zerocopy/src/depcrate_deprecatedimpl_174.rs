// Generated macro for impl_174 (impl)
macro_rules! Depcrate_deprecatedimpl_174 {
() => {
// Module: crate::deprecated
// Provides: {"impl_174"}
// Dependencies: {}
impl < B , T > Ref < B , T > where B : ByteSlice , T : KnownLayout + Immutable + ? Sized , { # [deprecated (since = "0.8.0" , note = "renamed to `Ref::from_bytes`")] # [doc (hidden)] # [must_use = "has no side effects"] # [inline (always)] pub fn new (bytes : B) -> Option < Ref < B , T > > { Self :: from_bytes (bytes) . ok () } }
};
}
