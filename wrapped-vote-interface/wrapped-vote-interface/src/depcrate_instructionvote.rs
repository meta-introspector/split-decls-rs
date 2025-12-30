// Generated macro for vote (function)
macro_rules! Depcrate_instructionvote {
() => {
// Module: crate::instruction
// Provides: {"vote"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn vote (vote_pubkey : & Pubkey , authorized_voter_pubkey : & Pubkey , vote : Vote) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new_readonly (sysvar :: slot_hashes :: id () , false) , AccountMeta :: new_readonly (sysvar :: clock :: id () , false) , AccountMeta :: new_readonly (* authorized_voter_pubkey , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: Vote (vote) , account_metas) }
};
}
