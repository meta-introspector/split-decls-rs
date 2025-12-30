// Generated macro for vote_switch (function)
macro_rules! Depcrate_instructionvote_switch {
() => {
// Module: crate::instruction
// Provides: {"vote_switch"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn vote_switch (vote_pubkey : & Pubkey , authorized_voter_pubkey : & Pubkey , vote : Vote , proof_hash : Hash ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new_readonly (sysvar :: slot_hashes :: id () , false) , AccountMeta :: new_readonly (sysvar :: clock :: id () , false) , AccountMeta :: new_readonly (* authorized_voter_pubkey , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: VoteSwitch (vote , proof_hash) , account_metas ,) }
};
}
