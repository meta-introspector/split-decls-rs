// Generated macro for impl_101 (impl)
macro_rules! Depcrate_state_vote_instruction_dataimpl_101 {
() => {
// Module: crate::state::vote_instruction_data
// Provides: {"impl_101"}
// Dependencies: {}
impl TowerSync { pub fn new (lockouts : VecDeque < Lockout > , root : Option < Slot > , hash : Hash , block_id : Hash ,) -> Self { Self { lockouts , root , hash , timestamp : None , block_id , } } # [doc = " Creates a tower with consecutive votes for `slot - MAX_LOCKOUT_HISTORY + 1` to `slot` inclusive."] # [doc = " If `slot >= MAX_LOCKOUT_HISTORY`, sets the root to `(slot - MAX_LOCKOUT_HISTORY)`"] # [doc = " Sets the hash to `hash` and leaves `block_id` unset."] pub fn new_from_slot (slot : Slot , hash : Hash) -> Self { let lowest_slot = slot . saturating_add (1) . saturating_sub (MAX_LOCKOUT_HISTORY as u64) ; let slots : Vec < _ > = (lowest_slot .. slot . saturating_add (1)) . collect () ; Self :: new_from_slots (slots , hash , (lowest_slot > 0) . then (| | lowest_slot . saturating_sub (1)) ,) } # [doc = " Creates a tower with consecutive confirmation for `slots`"] pub fn new_from_slots (slots : Vec < Slot > , hash : Hash , root : Option < Slot >) -> Self { let lockouts : VecDeque < Lockout > = slots . into_iter () . rev () . enumerate () . map (| (cc , s) | Lockout :: new_with_confirmation_count (s , cc . saturating_add (1) as u32)) . rev () . collect () ; Self { lockouts , hash , root , timestamp : None , block_id : Hash :: default () , } } pub fn slots (& self) -> Vec < Slot > { self . lockouts . iter () . map (| lockout | lockout . slot ()) . collect () } pub fn last_voted_slot (& self) -> Option < Slot > { self . lockouts . back () . map (| l | l . slot ()) } }
};
}
