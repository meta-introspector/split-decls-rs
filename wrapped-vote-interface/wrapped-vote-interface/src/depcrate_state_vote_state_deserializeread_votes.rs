// Generated macro for read_votes (function)
macro_rules! Depcrate_state_vote_state_deserializeread_votes {
() => {
// Module: crate::state::vote_state_deserialize
// Provides: {"read_votes"}
// Dependencies: {}
fn read_votes < T : AsRef < [u8] > > (cursor : & mut Cursor < T > , has_latency : bool ,) -> Result < VecDeque < LandedVote > , InstructionError > { let vote_count = read_u64 (cursor) ? as usize ; let mut votes = VecDeque :: with_capacity (vote_count . min (MAX_LOCKOUT_HISTORY)) ; for _ in 0 .. vote_count { let latency = if has_latency { read_u8 (cursor) ? } else { 0 } ; let slot = read_u64 (cursor) ? ; let confirmation_count = read_u32 (cursor) ? ; let lockout = Lockout :: new_with_confirmation_count (slot , confirmation_count) ; votes . push_back (LandedVote { latency , lockout }) ; } Ok (votes) }
};
}
