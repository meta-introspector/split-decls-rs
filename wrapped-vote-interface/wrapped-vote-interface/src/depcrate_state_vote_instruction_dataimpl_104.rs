// Generated macro for impl_104 (impl)
macro_rules! Depcrate_state_vote_instruction_dataimpl_104 {
() => {
// Module: crate::state::vote_instruction_data
// Provides: {"impl_104"}
// Dependencies: {}
impl Default for VoteInitV2 { fn default () -> Self { Self { node_pubkey : Pubkey :: default () , authorized_voter : Pubkey :: default () , authorized_voter_bls_pubkey : [0u8 ; BLS_PUBLIC_KEY_COMPRESSED_SIZE] , authorized_voter_bls_proof_of_possession : [0u8 ; BLS_PROOF_OF_POSSESSION_COMPRESSED_SIZE] , authorized_withdrawer : Pubkey :: default () , inflation_rewards_commission_bps : 0 , inflation_rewards_collector : Pubkey :: default () , block_revenue_commission_bps : 0 , block_revenue_collector : Pubkey :: default () , } } }
};
}
