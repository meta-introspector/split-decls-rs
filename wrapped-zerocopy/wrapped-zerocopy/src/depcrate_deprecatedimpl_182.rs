// Generated macro for impl_182 (impl)
macro_rules! Depcrate_deprecatedimpl_182 {
() => {
// Module: crate::deprecated
// Provides: {"impl_182"}
// Dependencies: {}
impl < 'a , B , T > Ref < B , [T] > where B : 'a + IntoByteSlice < 'a > , T : FromBytes + Immutable , { # [deprecated (since = "0.8.0" , note = "`Ref::into_ref` now supports slices")] # [doc (hidden)] # [inline (always)] pub fn into_slice (self) -> & 'a [T] { Ref :: into_ref (self) } }
};
}
