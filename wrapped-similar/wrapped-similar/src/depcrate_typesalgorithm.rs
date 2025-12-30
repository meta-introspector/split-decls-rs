// Generated macro for Algorithm (enum)
macro_rules! Depcrate_typesAlgorithm {
() => {
// Module: crate::types
// Provides: {"Algorithm"}
// Dependencies: {}
# [doc = " An enum representing a diffing algorithm."] # [derive (Clone , Copy , Hash , PartialEq , Eq , PartialOrd , Ord , Debug)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize) , serde (rename_all = "snake_case"))] pub enum Algorithm { # [doc = " Picks the myers algorithm from [`crate::algorithms::myers`]"] Myers , # [doc = " Picks the patience algorithm from [`crate::algorithms::patience`]"] Patience , # [doc = " Picks the LCS algorithm from [`crate::algorithms::lcs`]"] Lcs , }
};
}
