// Generated macro for tests (module)
macro_rules! Depcrate_state_vote_state_versionstests {
() => {
// Module: crate::state::vote_state_versions
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_vote_state_versions_deserialize () { let ser_deser = | original : VoteStateVersions | { let serialized = bincode :: serialize (& original) . unwrap () ; VoteStateVersions :: deserialize (& serialized) } ; let v1_14_11 = VoteStateVersions :: V1_14_11 (Box :: default ()) ; assert_eq ! (ser_deser (v1_14_11 . clone ()) , Ok (v1_14_11) ,) ; let v3 = VoteStateVersions :: V3 (Box :: default ()) ; assert_eq ! (ser_deser (v3 . clone ()) , Ok (v3) ,) ; let v4 = VoteStateVersions :: V4 (Box :: default ()) ; assert_eq ! (ser_deser (v4 . clone ()) , Ok (v4) ,) ; } }
};
}
