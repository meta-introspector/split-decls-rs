// Generated macro for impl_95 (impl)
macro_rules! Depcrate_state_vote_instruction_dataimpl_95 {
() => {
// Module: crate::state::vote_instruction_data
// Provides: {"impl_95"}
// Dependencies: {}
impl Vote { pub fn new (slots : Vec < Slot > , hash : Hash) -> Self { Self { slots , hash , timestamp : None , } } pub fn last_voted_slot (& self) -> Option < Slot > { self . slots . last () . copied () } }
};
}
