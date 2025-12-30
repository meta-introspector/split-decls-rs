// Generated macro for FoundConflict (enum)
macro_rules! Depcrate_cdclFoundConflict {
() => {
// Module: crate::cdcl
// Provides: {"FoundConflict"}
// Dependencies: {}
# [doc = " Return type of [`find_conflict`]."] # [doc = ""] # [doc = " Specifies whether a conflict was found during propagation or while enqueuing assumptions."] enum FoundConflict { Conflict (Conflict) , Assumption , }
};
}
