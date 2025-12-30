// Generated macro for compact_update_vote_state_switch (function)
macro_rules! Depcrate_instructioncompact_update_vote_state_switch {
() => {
// Module: crate::instruction
// Provides: {"compact_update_vote_state_switch"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn compact_update_vote_state_switch (vote_pubkey : & Pubkey , authorized_voter_pubkey : & Pubkey , vote_state_update : VoteStateUpdate , proof_hash : Hash ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new_readonly (* authorized_voter_pubkey , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: CompactUpdateVoteStateSwitch (vote_state_update , proof_hash) , account_metas ,) }
};
}
