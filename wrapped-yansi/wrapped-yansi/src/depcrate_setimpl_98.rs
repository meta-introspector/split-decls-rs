// Generated macro for impl_98 (impl)
macro_rules! Depcrate_setimpl_98 {
() => {
// Module: crate::set
// Provides: {"impl_98"}
// Dependencies: {}
impl < T : SetMember > Iterator for Iter < T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { while self . index <= T :: MAX_VALUE { let mask : u16 = 1 << self . index ; self . index += 1 ; if let Some (v) = T :: from_bit_mask (mask) { if self . set . contains (v) { return Some (v) ; } } } None } }
};
}
