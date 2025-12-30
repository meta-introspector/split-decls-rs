// Generated macro for Timestamp (struct)
macro_rules! Depcrate_utilTimestamp {
() => {
// Module: crate::util
// Provides: {"Timestamp"}
// Dependencies: {}
# [doc = " A Unix timestamp with nanoseconds precision"] # [derive (Serialize , Deserialize , Debug , Copy , Clone , Hash , PartialEq , Eq , PartialOrd , Ord)] pub struct Timestamp { seconds : i64 , # [doc = " Always in the `0 .. 1_000_000_000` range."] nanoseconds : u32 , }
};
}
