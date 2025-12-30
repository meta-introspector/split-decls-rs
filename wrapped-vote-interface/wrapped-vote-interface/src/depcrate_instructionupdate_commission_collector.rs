// Generated macro for update_commission_collector (function)
macro_rules! Depcrate_instructionupdate_commission_collector {
() => {
// Module: crate::instruction
// Provides: {"update_commission_collector"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn update_commission_collector (vote_pubkey : & Pubkey , authorized_withdrawer_pubkey : & Pubkey , new_collector_pubkey : & Pubkey , kind : CommissionKind ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new (* new_collector_pubkey , false) , AccountMeta :: new_readonly (* authorized_withdrawer_pubkey , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: UpdateCommissionCollector (kind) , account_metas ,) }
};
}
