// Generated macro for update_validator_identity (function)
macro_rules! Depcrate_instructionupdate_validator_identity {
() => {
// Module: crate::instruction
// Provides: {"update_validator_identity"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn update_validator_identity (vote_pubkey : & Pubkey , authorized_withdrawer_pubkey : & Pubkey , node_pubkey : & Pubkey ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new_readonly (* node_pubkey , true) , AccountMeta :: new_readonly (* authorized_withdrawer_pubkey , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: UpdateValidatorIdentity , account_metas ,) }
};
}
