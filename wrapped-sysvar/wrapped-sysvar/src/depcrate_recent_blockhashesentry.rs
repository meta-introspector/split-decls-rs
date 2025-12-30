// Generated macro for Entry (struct)
macro_rules! Depcrate_recent_blockhashesEntry {
() => {
// Module: crate::recent_blockhashes
// Provides: {"Entry"}
// Dependencies: {}
# [deprecated (since = "1.9.0" , note = "Please do not use, will no longer be available in the future")] # [repr (C)] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Clone , Debug , Default , PartialEq , Eq)] pub struct Entry { pub blockhash : Hash , pub fee_calculator : FeeCalculator , }
};
}
