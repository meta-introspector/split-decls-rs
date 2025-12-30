// Generated macro for read_epoch_credits (function)
macro_rules! Depcrate_state_vote_state_deserializeread_epoch_credits {
() => {
// Module: crate::state::vote_state_deserialize
// Provides: {"read_epoch_credits"}
// Dependencies: {}
fn read_epoch_credits < T : AsRef < [u8] > > (cursor : & mut Cursor < T > ,) -> Result < Vec < (Epoch , u64 , u64) > , InstructionError > { let epoch_credit_count = read_u64 (cursor) ? as usize ; let mut epoch_credits = Vec :: with_capacity (epoch_credit_count . min (MAX_EPOCH_CREDITS_HISTORY)) ; for _ in 0 .. epoch_credit_count { let epoch = read_u64 (cursor) ? ; let credits = read_u64 (cursor) ? ; let prev_credits = read_u64 (cursor) ? ; epoch_credits . push ((epoch , credits , prev_credits)) ; } Ok (epoch_credits) }
};
}
