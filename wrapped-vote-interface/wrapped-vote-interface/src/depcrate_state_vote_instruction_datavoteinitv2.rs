// Generated macro for VoteInitV2 (struct)
macro_rules! Depcrate_state_vote_instruction_dataVoteInitV2 {
() => {
// Module: crate::state::vote_instruction_data
// Provides: {"VoteInitV2"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , cfg_eval :: cfg_eval , serde_as)] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Debug , PartialEq , Eq , Clone , Copy)] pub struct VoteInitV2 { pub node_pubkey : Pubkey , pub authorized_voter : Pubkey , # [cfg_attr (feature = "serde" , serde_as (as = "[_; BLS_PUBLIC_KEY_COMPRESSED_SIZE]"))] pub authorized_voter_bls_pubkey : [u8 ; BLS_PUBLIC_KEY_COMPRESSED_SIZE] , # [cfg_attr (feature = "serde" , serde_as (as = "[_; BLS_PROOF_OF_POSSESSION_COMPRESSED_SIZE]"))] pub authorized_voter_bls_proof_of_possession : [u8 ; BLS_PROOF_OF_POSSESSION_COMPRESSED_SIZE] , pub authorized_withdrawer : Pubkey , pub inflation_rewards_commission_bps : u16 , pub inflation_rewards_collector : Pubkey , pub block_revenue_commission_bps : u16 , pub block_revenue_collector : Pubkey , }
};
}
