// Generated macro for tests (module)
macro_rules! Depcrate_state_vote_state_deserializetests {
() => {
// Module: crate::state::vote_state_deserialize
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; const PRIOR_VOTERS_SIZE : usize = MAX_ITEMS * core :: mem :: size_of :: < (Pubkey , Epoch , Epoch) > () + core :: mem :: size_of :: < u64 > () + core :: mem :: size_of :: < bool > () ; # [test] fn test_skip_prior_voters_success () { let buffer = vec ! [0u8 ; PRIOR_VOTERS_SIZE] ; let mut cursor = Cursor :: new (& buffer [..]) ; let result = skip_prior_voters (& mut cursor) ; assert ! (result . is_ok ()) ; assert_eq ! (cursor . position () as usize , PRIOR_VOTERS_SIZE) ; } # [test] fn test_skip_prior_voters_success_with_offset () { let offset = 100 ; let buffer = vec ! [0u8 ; PRIOR_VOTERS_SIZE + offset] ; let mut cursor = Cursor :: new (& buffer [..]) ; cursor . set_position (offset as u64) ; let result = skip_prior_voters (& mut cursor) ; assert ! (result . is_ok ()) ; assert_eq ! (cursor . position () as usize , PRIOR_VOTERS_SIZE + offset) ; } # [test] fn test_skip_prior_voters_buffer_too_small () { let buffer = vec ! [0u8 ; PRIOR_VOTERS_SIZE - 1] ; let mut cursor = Cursor :: new (& buffer [..]) ; let result = skip_prior_voters (& mut cursor) ; assert_eq ! (result , Err (InstructionError :: InvalidAccountData)) ; } # [test] fn test_skip_prior_voters_insufficient_remaining () { let buffer = vec ! [0u8 ; PRIOR_VOTERS_SIZE + 100] ; let mut cursor = Cursor :: new (& buffer [..]) ; cursor . set_position (101) ; let result = skip_prior_voters (& mut cursor) ; assert_eq ! (result , Err (InstructionError :: InvalidAccountData)) ; } }
};
}
