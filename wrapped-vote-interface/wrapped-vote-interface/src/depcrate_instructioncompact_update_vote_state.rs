// Generated macro for compact_update_vote_state (function)
macro_rules! Depcrate_instructioncompact_update_vote_state {
() => {
// Module: crate::instruction
// Provides: {"compact_update_vote_state"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn compact_update_vote_state (vote_pubkey : & Pubkey , authorized_voter_pubkey : & Pubkey , vote_state_update : VoteStateUpdate ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new_readonly (* authorized_voter_pubkey , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: CompactUpdateVoteState (vote_state_update) , account_metas ,) }
};
}
