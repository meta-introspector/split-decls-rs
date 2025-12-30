// Generated macro for ChangeTag (enum)
macro_rules! Depcrate_typesChangeTag {
() => {
// Module: crate::types
// Provides: {"ChangeTag"}
// Dependencies: {}
# [doc = " The tag of a change."] # [derive (Debug , PartialEq , Eq , Hash , Clone , Copy , Ord , PartialOrd)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize) , serde (rename_all = "snake_case"))] pub enum ChangeTag { # [doc = " The change indicates equality (not a change)"] Equal , # [doc = " The change indicates deleted text."] Delete , # [doc = " The change indicates inserted text."] Insert , }
};
}
