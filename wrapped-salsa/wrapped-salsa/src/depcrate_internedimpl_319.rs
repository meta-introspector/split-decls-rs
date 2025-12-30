// Generated macro for impl_319 (impl)
macro_rules! Depcrate_internedimpl_319 {
() => {
// Module: crate::interned
// Provides: {"impl_319"}
// Dependencies: {}
impl ValueShared { # [doc = " Returns `true` if this value slot can be reused when interning, and should be added to the LRU."] fn is_reusable < C : Configuration > (& self) -> bool { if C :: REVISIONS == IMMORTAL { return false ; } self . durability == Durability :: LOW } }
};
}
