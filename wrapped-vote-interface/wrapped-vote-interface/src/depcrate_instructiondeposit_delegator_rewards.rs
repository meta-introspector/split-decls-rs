// Generated macro for deposit_delegator_rewards (function)
macro_rules! Depcrate_instructiondeposit_delegator_rewards {
() => {
// Module: crate::instruction
// Provides: {"deposit_delegator_rewards"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn deposit_delegator_rewards (vote_pubkey : & Pubkey , source_pubkey : & Pubkey , deposit : u64 ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new (* source_pubkey , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: DepositDelegatorRewards { deposit } , account_metas ,) }
};
}
