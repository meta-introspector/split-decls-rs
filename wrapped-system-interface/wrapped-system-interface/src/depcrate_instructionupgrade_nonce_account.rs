// Generated macro for upgrade_nonce_account (function)
macro_rules! Depcrate_instructionupgrade_nonce_account {
() => {
// Module: crate::instruction
// Provides: {"upgrade_nonce_account"}
// Dependencies: {}
# [doc = " One-time idempotent upgrade of legacy nonce versions in order to bump"] # [doc = " them out of chain blockhash domain."] # [cfg (feature = "bincode")] pub fn upgrade_nonce_account (nonce_address : Address) -> Instruction { let account_metas = vec ! [AccountMeta :: new (nonce_address , false)] ; Instruction :: new_with_bincode (ID , & SystemInstruction :: UpgradeNonceAccount , account_metas) }
};
}
