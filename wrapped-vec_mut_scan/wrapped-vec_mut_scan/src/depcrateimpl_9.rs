// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl < 's , 'a , T : 'a > Deref for VecMutScanItem < 's , 'a , T > { type Target = T ; fn deref (& self) -> & Self :: Target { unsafe { & * self . scan . base . add (self . scan . read) } } }
};
}
