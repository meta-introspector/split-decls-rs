// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl < 's , 'a , T : 'a > DerefMut for VecGrowScanItem < 's , 'a , T > { fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { & mut * self . scan . base . add (self . scan . read) } } }
};
}
