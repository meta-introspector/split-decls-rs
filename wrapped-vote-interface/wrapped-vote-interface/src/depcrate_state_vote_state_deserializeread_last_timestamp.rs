// Generated macro for read_last_timestamp (function)
macro_rules! Depcrate_state_vote_state_deserializeread_last_timestamp {
() => {
// Module: crate::state::vote_state_deserialize
// Provides: {"read_last_timestamp"}
// Dependencies: {}
fn read_last_timestamp < T : AsRef < [u8] > > (cursor : & mut Cursor < T > ,) -> Result < BlockTimestamp , InstructionError > { let slot = read_u64 (cursor) ? ; let timestamp = read_i64 (cursor) ? ; Ok (BlockTimestamp { slot , timestamp }) }
};
}
