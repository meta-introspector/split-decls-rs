// Generated macro for tower_sync (function)
macro_rules! Depcrate_instructiontower_sync {
() => {
// Module: crate::instruction
// Provides: {"tower_sync"}
// Dependencies: {}
# [cfg (feature = "bincode")] pub fn tower_sync (vote_pubkey : & Pubkey , authorized_voter_pubkey : & Pubkey , tower_sync : TowerSync ,) -> Instruction { let account_metas = vec ! [AccountMeta :: new (* vote_pubkey , false) , AccountMeta :: new_readonly (* authorized_voter_pubkey , true) ,] ; Instruction :: new_with_bincode (id () , & VoteInstruction :: TowerSync (tower_sync) , account_metas) }
};
}
