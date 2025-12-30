// Generated macro for impl_11 (impl)
macro_rules! Depcrateimpl_11 {
() => {
// Module: crate
// Provides: {"impl_11"}
// Dependencies: {}
impl SlotHistory { pub fn add (& mut self , slot : u64) { if slot > self . next_slot && slot - self . next_slot >= MAX_ENTRIES { let full_blocks = (MAX_ENTRIES as usize) / 64 ; for i in 0 .. full_blocks { self . bits . set_block (i , 0) ; } } else { for skipped in self . next_slot .. slot { self . bits . set (skipped % MAX_ENTRIES , false) ; } } self . bits . set (slot % MAX_ENTRIES , true) ; self . next_slot = slot + 1 ; } pub fn check (& self , slot : u64) -> Check { if slot > self . newest () { Check :: Future } else if slot < self . oldest () { Check :: TooOld } else if self . bits . get (slot % MAX_ENTRIES) { Check :: Found } else { Check :: NotFound } } pub fn oldest (& self) -> u64 { self . next_slot . saturating_sub (MAX_ENTRIES) } pub fn newest (& self) -> u64 { self . next_slot - 1 } }
};
}
