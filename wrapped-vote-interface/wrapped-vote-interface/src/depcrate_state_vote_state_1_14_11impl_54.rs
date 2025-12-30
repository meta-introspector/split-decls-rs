// Generated macro for impl_54 (impl)
macro_rules! Depcrate_state_vote_state_1_14_11impl_54 {
() => {
// Module: crate::state::vote_state_1_14_11
// Provides: {"impl_54"}
// Dependencies: {}
impl VoteState1_14_11 { pub fn get_rent_exempt_reserve (rent : & Rent) -> u64 { rent . minimum_balance (Self :: size_of ()) } # [doc = " Upper limit on the size of the Vote State"] # [doc = " when votes.len() is MAX_LOCKOUT_HISTORY."] pub fn size_of () -> usize { 3731 } pub fn is_uninitialized (& self) -> bool { self . authorized_voters . is_empty () } pub fn is_correct_size_and_initialized (data : & [u8]) -> bool { const VERSION_OFFSET : usize = 4 ; const DEFAULT_PRIOR_VOTERS_END : usize = VERSION_OFFSET + DEFAULT_PRIOR_VOTERS_OFFSET ; data . len () == VoteState1_14_11 :: size_of () && data [VERSION_OFFSET .. DEFAULT_PRIOR_VOTERS_END] != [0 ; DEFAULT_PRIOR_VOTERS_OFFSET] } }
};
}
