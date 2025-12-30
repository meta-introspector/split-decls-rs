// Generated macro for deserialize_vote_state_into_v1_14_11 (function)
macro_rules! Depcrate_state_vote_state_deserializedeserialize_vote_state_into_v1_14_11 {
() => {
// Module: crate::state::vote_state_deserialize
// Provides: {"deserialize_vote_state_into_v1_14_11"}
// Dependencies: {}
pub (crate) fn deserialize_vote_state_into_v1_14_11 (cursor : & mut Cursor < & [u8] > , vote_state : * mut crate :: state :: vote_state_1_14_11 :: VoteState1_14_11 ,) -> Result < () , InstructionError > { read_pubkey_into (cursor , unsafe { addr_of_mut ! ((* vote_state) . node_pubkey) } ,) ? ; read_pubkey_into (cursor , unsafe { addr_of_mut ! ((* vote_state) . authorized_withdrawer) } ,) ? ; let commission = read_u8 (cursor) ? ; let votes = read_votes_as_lockouts (cursor) ? ; let root_slot = read_option_u64 (cursor) ? ; let authorized_voters = read_authorized_voters (cursor) ? ; read_prior_voters_into (cursor , unsafe { addr_of_mut ! ((* vote_state) . prior_voters) } ,) ? ; let epoch_credits = read_epoch_credits (cursor) ? ; let last_timestamp = read_last_timestamp (cursor) ? ; unsafe { addr_of_mut ! ((* vote_state) . commission) . write (commission) ; addr_of_mut ! ((* vote_state) . votes) . write (votes) ; addr_of_mut ! ((* vote_state) . root_slot) . write (root_slot) ; addr_of_mut ! ((* vote_state) . authorized_voters) . write (authorized_voters) ; addr_of_mut ! ((* vote_state) . epoch_credits) . write (epoch_credits) ; addr_of_mut ! ((* vote_state) . last_timestamp) . write (last_timestamp) ; } Ok (()) }
};
}
