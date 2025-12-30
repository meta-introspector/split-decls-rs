// Generated macro for impl_138 (impl)
macro_rules! Depcrate_stateimpl_138 {
() => {
// Module: crate::state
// Provides: {"impl_138"}
// Dependencies: {}
impl Lockout { pub fn new (slot : Slot) -> Self { Self :: new_with_confirmation_count (slot , 1) } pub fn new_with_confirmation_count (slot : Slot , confirmation_count : u32) -> Self { Self { slot , confirmation_count , } } pub fn lockout (& self) -> u64 { (INITIAL_LOCKOUT as u64) . wrapping_pow (std :: cmp :: min (self . confirmation_count () , MAX_LOCKOUT_HISTORY as u32 ,)) } pub fn last_locked_out_slot (& self) -> Slot { self . slot . saturating_add (self . lockout ()) } pub fn is_locked_out_at_slot (& self , slot : Slot) -> bool { self . last_locked_out_slot () >= slot } pub fn slot (& self) -> Slot { self . slot } pub fn confirmation_count (& self) -> u32 { self . confirmation_count } pub fn increase_confirmation_count (& mut self , by : u32) { self . confirmation_count = self . confirmation_count . saturating_add (by) } }
};
}
