// Generated macro for impl_343 (impl)
macro_rules! Depcrate_internedimpl_343 {
() => {
// Module: crate::interned
// Provides: {"impl_343"}
// Dependencies: {}
impl < 'a , T > Lookup < Arc < T > > for & 'a T where T : ? Sized + Hash + Eq , Arc < T > : From < & 'a T > , { fn into_owned (self) -> Arc < T > { Arc :: from (self) } }
};
}
