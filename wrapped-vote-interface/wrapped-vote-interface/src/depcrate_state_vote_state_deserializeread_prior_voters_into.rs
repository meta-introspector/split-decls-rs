// Generated macro for read_prior_voters_into (function)
macro_rules! Depcrate_state_vote_state_deserializeread_prior_voters_into {
() => {
// Module: crate::state::vote_state_deserialize
// Provides: {"read_prior_voters_into"}
// Dependencies: {}
fn read_prior_voters_into < T : AsRef < [u8] > > (cursor : & mut Cursor < T > , prior_voters : * mut crate :: state :: CircBuf < (Pubkey , Epoch , Epoch) > ,) -> Result < () , InstructionError > { unsafe { let prior_voters_buf = addr_of_mut ! ((* prior_voters) . buf) as * mut (Pubkey , Epoch , Epoch) ; for i in 0 .. MAX_ITEMS { let prior_voter = read_pubkey (cursor) ? ; let from_epoch = read_u64 (cursor) ? ; let until_epoch = read_u64 (cursor) ? ; prior_voters_buf . add (i) . write ((prior_voter , from_epoch , until_epoch)) ; } (* prior_voters) . idx = read_u64 (cursor) ? as usize ; (* prior_voters) . is_empty = read_bool (cursor) ? ; } Ok (()) }
};
}
