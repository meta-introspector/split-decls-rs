// Generated macro for impl_65 (impl)
macro_rules! Depcrate_state_vote_state_versionsimpl_65 {
() => {
// Module: crate::state::vote_state_versions
// Provides: {"impl_65"}
// Dependencies: {}
# [cfg (test)] impl Arbitrary < '_ > for VoteStateVersions { fn arbitrary (u : & mut Unstructured < '_ >) -> arbitrary :: Result < Self > { let variant = u . choose_index (3) ? ; match variant { 0 => Ok (Self :: V4 (Box :: new (VoteStateV4 :: arbitrary (u) ?))) , 1 => Ok (Self :: V3 (Box :: new (VoteStateV3 :: arbitrary (u) ?))) , 2 => Ok (Self :: V1_14_11 (Box :: new (VoteState1_14_11 :: arbitrary (u) ?))) , _ => unreachable ! () , } } }
};
}
