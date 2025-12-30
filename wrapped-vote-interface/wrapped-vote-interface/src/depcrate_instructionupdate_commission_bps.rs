// Generated macro for update_commission_bps (function)
macro_rules! Depcrate_instructionupdate_commission_bps {
() => {
// Module: crate::instruction
// Provides: {"update_commission_bps"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn update_commission_bps (vote_pubkey : & Pubkey , authorized_withdrawer_pubkey : & Pubkey , kind : CommissionKind , commission_bps : u16 ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new_readonly (* authorized_withdrawer_pubkey , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: UpdateCommissionBps { kind , commission_bps , } , account_metas ,) }
};
}
