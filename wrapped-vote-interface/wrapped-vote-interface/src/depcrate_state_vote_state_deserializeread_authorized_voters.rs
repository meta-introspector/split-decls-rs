// Generated macro for read_authorized_voters (function)
macro_rules! Depcrate_state_vote_state_deserializeread_authorized_voters {
() => {
// Module: crate::state::vote_state_deserialize
// Provides: {"read_authorized_voters"}
// Dependencies: {}
fn read_authorized_voters < T : AsRef < [u8] > > (cursor : & mut Cursor < T > ,) -> Result < AuthorizedVoters , InstructionError > { let authorized_voter_count = read_u64 (cursor) ? ; let mut authorized_voters = AuthorizedVoters :: default () ; for _ in 0 .. authorized_voter_count { let epoch = read_u64 (cursor) ? ; let authorized_voter = read_pubkey (cursor) ? ; authorized_voters . insert (epoch , authorized_voter) ; } Ok (authorized_voters) }
};
}
