// Generated macro for VersionedTransaction (struct)
macro_rules! Depcrate_versionedVersionedTransaction {
() => {
// Module: crate::versioned
// Provides: {"VersionedTransaction"}
// Dependencies: {}
# [doc = " An atomic transaction"] # [cfg_attr (feature = "frozen-abi" , derive (solana_frozen_abi_macro :: AbiExample))] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [cfg_attr (feature = "wincode" , derive (SchemaWrite , SchemaRead))] # [derive (Debug , PartialEq , Default , Eq , Clone)] pub struct VersionedTransaction { # [doc = " List of signatures"] # [cfg_attr (feature = "serde" , serde (with = "short_vec"))] # [cfg_attr (feature = "wincode" , wincode (with = "containers::Vec<_, ShortU16Len>"))] pub signatures : Vec < Signature > , # [doc = " Message to sign."] pub message : VersionedMessage , }
};
}
