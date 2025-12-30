// Generated macro for AuthorizedVoters (struct)
macro_rules! Depcrate_authorized_votersAuthorizedVoters {
() => {
// Module: crate::authorized_voters
// Provides: {"AuthorizedVoters"}
// Dependencies: {}
# [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Debug , Default , PartialEq , Eq , Clone)] # [cfg_attr (feature = "dev-context-only-utils" , derive (Arbitrary))] pub struct AuthorizedVoters { authorized_voters : BTreeMap < Epoch , Pubkey > , }
};
}
