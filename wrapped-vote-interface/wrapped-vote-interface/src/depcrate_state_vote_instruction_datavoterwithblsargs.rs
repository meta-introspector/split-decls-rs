// Generated macro for VoterWithBLSArgs (struct)
macro_rules! Depcrate_state_vote_instruction_dataVoterWithBLSArgs {
() => {
// Module: crate::state::vote_instruction_data
// Provides: {"VoterWithBLSArgs"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , cfg_eval :: cfg_eval , serde_as)] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Debug , PartialEq , Eq , Clone , Copy)] pub struct VoterWithBLSArgs { # [cfg_attr (feature = "serde" , serde_as (as = "[_; BLS_PUBLIC_KEY_COMPRESSED_SIZE]"))] pub bls_pubkey : [u8 ; BLS_PUBLIC_KEY_COMPRESSED_SIZE] , # [cfg_attr (feature = "serde" , serde_as (as = "[_; BLS_PROOF_OF_POSSESSION_COMPRESSED_SIZE]"))] pub bls_proof_of_possession : [u8 ; BLS_PROOF_OF_POSSESSION_COMPRESSED_SIZE] , }
};
}
