// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl < 's , 'a , T : 'a > Deref for VecGrowScanItem < 's , 'a , T > { type Target = T ; fn deref (& self) -> & Self :: Target { unsafe { & * self . scan . base . add (self . scan . read) } } }
};
}
