// Generated macro for impl_10 (impl)
macro_rules! Depcrateimpl_10 {
() => {
// Module: crate
// Provides: {"impl_10"}
// Dependencies: {}
impl < 's , 'a , T : 'a > DerefMut for VecMutScanItem < 's , 'a , T > { fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { & mut * self . scan . base . add (self . scan . read) } } }
};
}
