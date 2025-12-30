// Generated macro for VoteInit (struct)
macro_rules! Depcrate_state_vote_instruction_dataVoteInit {
() => {
// Module: crate::state::vote_instruction_data
// Provides: {"VoteInit"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Default , Debug , PartialEq , Eq , Clone , Copy)] pub struct VoteInit { pub node_pubkey : Pubkey , pub authorized_voter : Pubkey , pub authorized_withdrawer : Pubkey , pub commission : u8 , }
};
}
