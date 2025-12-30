// Generated macro for TransactionVersion (enum)
macro_rules! Depcrate_versionedTransactionVersion {
() => {
// Module: crate::versioned
// Provides: {"TransactionVersion"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (Deserialize , Serialize) , serde (rename_all = "camelCase" , untagged))] # [derive (Clone , Debug , PartialEq , Eq)] pub enum TransactionVersion { Legacy (Legacy) , Number (u8) , }
};
}
