// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl Default for SlotHistory { fn default () -> Self { let mut bits = BitVec :: new_fill (false , MAX_ENTRIES) ; bits . set (0 , true) ; Self { bits , next_slot : 1 } } }
};
}
