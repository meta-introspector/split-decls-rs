// Generated macro for impl_97 (impl)
macro_rules! Depcrate_state_vote_instruction_dataimpl_97 {
() => {
// Module: crate::state::vote_instruction_data
// Provides: {"impl_97"}
// Dependencies: {}
impl From < Vec < (Slot , u32) > > for VoteStateUpdate { fn from (recent_slots : Vec < (Slot , u32) >) -> Self { let lockouts : VecDeque < Lockout > = recent_slots . into_iter () . map (| (slot , confirmation_count) | { Lockout :: new_with_confirmation_count (slot , confirmation_count) }) . collect () ; Self { lockouts , root : None , hash : Hash :: default () , timestamp : None , } } }
};
}
