// Generated macro for update_vote_state (function)
macro_rules! Depcrate_instructionupdate_vote_state {
() => {
// Module: crate::instruction
// Provides: {"update_vote_state"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn update_vote_state (vote_pubkey : & Pubkey , authorized_voter_pubkey : & Pubkey , vote_state_update : VoteStateUpdate ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new_readonly (* authorized_voter_pubkey , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: UpdateVoteState (vote_state_update) , account_metas ,) }
};
}
