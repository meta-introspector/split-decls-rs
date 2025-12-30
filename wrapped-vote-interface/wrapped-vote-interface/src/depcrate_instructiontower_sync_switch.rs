// Generated macro for tower_sync_switch (function)
macro_rules! Depcrate_instructiontower_sync_switch {
() => {
// Module: crate::instruction
// Provides: {"tower_sync_switch"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn tower_sync_switch (vote_pubkey : & Pubkey , authorized_voter_pubkey : & Pubkey , tower_sync : TowerSync , proof_hash : Hash ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new_readonly (* authorized_voter_pubkey , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: TowerSyncSwitch (tower_sync , proof_hash) , account_metas ,) }
};
}
