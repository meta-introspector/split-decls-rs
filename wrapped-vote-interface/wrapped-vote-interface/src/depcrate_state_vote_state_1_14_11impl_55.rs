// Generated macro for impl_55 (impl)
macro_rules! Depcrate_state_vote_state_1_14_11impl_55 {
() => {
// Module: crate::state::vote_state_1_14_11
// Provides: {"impl_55"}
// Dependencies: {}
impl From < VoteStateV3 > for VoteState1_14_11 { fn from (vote_state : VoteStateV3) -> Self { Self { node_pubkey : vote_state . node_pubkey , authorized_withdrawer : vote_state . authorized_withdrawer , commission : vote_state . commission , votes : vote_state . votes . into_iter () . map (| landed_vote | landed_vote . into ()) . collect () , root_slot : vote_state . root_slot , authorized_voters : vote_state . authorized_voters , prior_voters : vote_state . prior_voters , epoch_credits : vote_state . epoch_credits , last_timestamp : vote_state . last_timestamp , } } }
};
}
