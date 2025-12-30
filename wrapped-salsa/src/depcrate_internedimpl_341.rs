// Generated macro for impl_341 (impl)
macro_rules! Depcrate_internedimpl_341 {
() => {
// Module: crate::interned
// Provides: {"impl_341"}
// Dependencies: {}
impl < 'a , T > Lookup < Box < T > > for & 'a T where T : ? Sized + Hash + Eq , Box < T > : From < & 'a T > , { fn into_owned (self) -> Box < T > { Box :: from (self) } }
};
}
