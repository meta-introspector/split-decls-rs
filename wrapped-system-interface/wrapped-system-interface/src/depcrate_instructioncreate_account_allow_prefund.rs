// Generated macro for create_account_allow_prefund (function)
macro_rules! Depcrate_instructioncreate_account_allow_prefund {
() => {
// Module: crate::instruction
// Provides: {"create_account_allow_prefund"}
// Dependencies: {}
# [doc = " Create a new account without enforcing zero lamports on the destination"] # [doc = " account."] # [doc = ""] # [doc = " # Required signers"] # [doc = ""] # [doc = " The `new_account_address` signer must sign the transaction. If present,"] # [doc = " the payer in `payer_and_lamports` must also sign the transaction."] # [cfg (feature = "bincode")] pub fn create_account_allow_prefund (new_account_address : & Address , payer_and_lamports : Option < (& Address , u64) > , space : u64 , owner : & Address ,) -> Instruction { let mut account_metas = vec ! [AccountMeta :: new (* new_account_address , true)] ; let lamports = match payer_and_lamports { None => 0 , Some ((from , lamports)) => { account_metas . push (AccountMeta :: new (* from , true)) ; lamports } } ; Instruction :: new_with_bincode (ID , & SystemInstruction :: CreateAccountAllowPrefund { lamports , space , owner : * owner , } , account_metas ,) }
};
}
