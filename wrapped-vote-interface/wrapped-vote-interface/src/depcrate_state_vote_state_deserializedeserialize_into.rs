// Generated macro for deserialize_into (function)
macro_rules! Depcrate_state_vote_state_deserializedeserialize_into {
() => {
// Module: crate::state::vote_state_deserialize
// Provides: {"deserialize_into"}
// Dependencies: {}
pub (crate) fn deserialize_into < T : Default > (input : & [u8] , vote_state : & mut T , deserialize_fn : impl FnOnce (& [u8] , * mut T) -> Result < () , InstructionError > ,) -> Result < () , InstructionError > { let vote_state = vote_state as * mut T ; unsafe { std :: ptr :: drop_in_place (vote_state) ; } let guard = DropGuard { vote_state } ; let res = deserialize_fn (input , vote_state) ; if res . is_ok () { std :: mem :: forget (guard) ; } res }
};
}
