// Generated macro for impl_137 (impl)
macro_rules! Depcrate_latchimpl_137 {
() => {
// Module: crate::latch
// Provides: {"impl_137"}
// Dependencies: {}
impl < L > Deref for LatchRef < '_ , L > { type Target = L ; fn deref (& self) -> & L { unsafe { & * self . inner } } }
};
}
