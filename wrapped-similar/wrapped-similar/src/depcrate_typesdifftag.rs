// Generated macro for DiffTag (enum)
macro_rules! Depcrate_typesDiffTag {
() => {
// Module: crate::types
// Provides: {"DiffTag"}
// Dependencies: {}
# [doc = " The tag of a diff operation."] # [derive (Debug , PartialEq , Eq , Hash , Clone , Copy , Ord , PartialOrd)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize) , serde (rename_all = "snake_case"))] pub enum DiffTag { # [doc = " The diff op encodes an equal segment."] Equal , # [doc = " The diff op encodes a deleted segment."] Delete , # [doc = " The diff op encodes an inserted segment."] Insert , # [doc = " The diff op encodes a replaced segment."] Replace , }
};
}
