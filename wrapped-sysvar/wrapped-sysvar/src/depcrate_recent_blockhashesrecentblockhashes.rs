// Generated macro for RecentBlockhashes (struct)
macro_rules! Depcrate_recent_blockhashesRecentBlockhashes {
() => {
// Module: crate::recent_blockhashes
// Provides: {"RecentBlockhashes"}
// Dependencies: {}
# [doc = " Contains recent block hashes and fee calculators."] # [doc = ""] # [doc = " The entries are ordered by descending block height, so the first entry holds"] # [doc = " the most recent block hash, and the last entry holds an old block hash."] # [deprecated (since = "1.9.0" , note = "Please do not use, will no longer be available in the future")] # [repr (C)] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Clone , Debug , PartialEq , Eq)] pub struct RecentBlockhashes (Vec < Entry >) ;
};
}
