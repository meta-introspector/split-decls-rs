// Generated macro for skip_prior_voters (function)
macro_rules! Depcrate_state_vote_state_deserializeskip_prior_voters {
() => {
// Module: crate::state::vote_state_deserialize
// Provides: {"skip_prior_voters"}
// Dependencies: {}
fn skip_prior_voters < T : AsRef < [u8] > > (cursor : & mut Cursor < T >) -> Result < () , InstructionError > { const PRIOR_VOTERS_SIZE : usize = MAX_ITEMS * core :: mem :: size_of :: < (Pubkey , Epoch , Epoch) > () + core :: mem :: size_of :: < u64 > () + core :: mem :: size_of :: < bool > () ; cursor . consume (PRIOR_VOTERS_SIZE) ; let bytes = cursor . get_ref () . as_ref () ; if cursor . position () as usize > bytes . len () { return Err (InstructionError :: InvalidAccountData) ; } Ok (()) }
};
}
