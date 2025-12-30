// Generated macro for VoteStateV3 (struct)
macro_rules! Depcrate_state_vote_state_v3VoteStateV3 {
() => {
// Module: crate::state::vote_state_v3
// Provides: {"VoteStateV3"}
// Dependencies: {}
# [cfg_attr (feature = "frozen-abi" , frozen_abi (digest = "pZqasQc6duzMYzpzU7eriHH9cMXmubuUP4NmCrkWZjt") , derive (AbiExample))] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Debug , Default , PartialEq , Eq , Clone)] # [cfg_attr (feature = "dev-context-only-utils" , derive (Arbitrary))] pub struct VoteStateV3 { # [doc = " the node that votes in this account"] pub node_pubkey : Pubkey , # [doc = " the signer for withdrawals"] pub authorized_withdrawer : Pubkey , # [doc = " percentage (0-100) that represents what part of a rewards"] # [doc = "  payout should be given to this VoteAccount"] pub commission : u8 , pub votes : VecDeque < LandedVote > , pub root_slot : Option < Slot > , # [doc = " the signer for vote transactions"] pub authorized_voters : AuthorizedVoters , # [doc = " history of prior authorized voters and the epochs for which"] # [doc = " they were set, the bottom end of the range is inclusive,"] # [doc = " the top of the range is exclusive"] pub prior_voters : CircBuf < (Pubkey , Epoch , Epoch) > , # [doc = " history of how many credits earned by the end of each epoch"] # [doc = "  each tuple is (Epoch, credits, prev_credits)"] pub epoch_credits : Vec < (Epoch , u64 , u64) > , # [doc = " most recent timestamp submitted with a vote"] pub last_timestamp : BlockTimestamp , }
};
}
