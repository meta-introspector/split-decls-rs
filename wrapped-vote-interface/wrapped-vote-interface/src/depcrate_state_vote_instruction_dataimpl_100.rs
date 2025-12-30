// Generated macro for impl_100 (impl)
macro_rules! Depcrate_state_vote_instruction_dataimpl_100 {
() => {
// Module: crate::state::vote_instruction_data
// Provides: {"impl_100"}
// Dependencies: {}
impl From < Vec < (Slot , u32) > > for TowerSync { fn from (recent_slots : Vec < (Slot , u32) >) -> Self { let lockouts : VecDeque < Lockout > = recent_slots . into_iter () . map (| (slot , confirmation_count) | { Lockout :: new_with_confirmation_count (slot , confirmation_count) }) . collect () ; Self { lockouts , root : None , hash : Hash :: default () , timestamp : None , block_id : Hash :: default () , } } }
};
}
