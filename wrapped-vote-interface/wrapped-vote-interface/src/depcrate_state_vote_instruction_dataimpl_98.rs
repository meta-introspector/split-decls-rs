// Generated macro for impl_98 (impl)
macro_rules! Depcrate_state_vote_instruction_dataimpl_98 {
() => {
// Module: crate::state::vote_instruction_data
// Provides: {"impl_98"}
// Dependencies: {}
impl VoteStateUpdate { pub fn new (lockouts : VecDeque < Lockout > , root : Option < Slot > , hash : Hash) -> Self { Self { lockouts , root , hash , timestamp : None , } } pub fn slots (& self) -> Vec < Slot > { self . lockouts . iter () . map (| lockout | lockout . slot ()) . collect () } pub fn last_voted_slot (& self) -> Option < Slot > { self . lockouts . back () . map (| l | l . slot ()) } }
};
}
