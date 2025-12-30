// Generated macro for use_112 (use)
macro_rules! Depcrate_state_vote_state_deserializeuse_112 {
() => {
// Module: crate::state::vote_state_deserialize
// Provides: {"use_112"}
// Dependencies: {}
use { crate :: { authorized_voters :: AuthorizedVoters , state :: { BlockTimestamp , LandedVote , Lockout , VoteStateV3 , VoteStateV4 , BLS_PUBLIC_KEY_COMPRESSED_SIZE , MAX_EPOCH_CREDITS_HISTORY , MAX_ITEMS , MAX_LOCKOUT_HISTORY , } , } , solana_clock :: Epoch , solana_instruction_error :: InstructionError , solana_pubkey :: Pubkey , solana_serialize_utils :: cursor :: { read_bool , read_i64 , read_option_u64 , read_pubkey , read_pubkey_into , read_u16 , read_u32 , read_u64 , read_u8 , } , std :: { collections :: VecDeque , io :: { BufRead , Cursor , Read } , ptr :: addr_of_mut , } , } ;
};
}
