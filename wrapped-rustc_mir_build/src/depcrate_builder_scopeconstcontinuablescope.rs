// Generated macro for ConstContinuableScope (struct)
macro_rules! Depcrate_builder_scopeConstContinuableScope {
() => {
// Module: crate::builder::scope
// Provides: {"ConstContinuableScope"}
// Dependencies: {}
# [derive (Debug)] struct ConstContinuableScope < 'tcx > { # [doc = " The scope for the `#[loop_match]` which its `#[const_continue]`s will jump to."] region_scope : region :: Scope , # [doc = " The place of the state of a `#[loop_match]`, which a `#[const_continue]` must update."] state_place : Place < 'tcx > , arms : Box < [ArmId] > , built_match_tree : BuiltMatchTree < 'tcx > , # [doc = " Drops that happen on a `#[const_continue]`"] const_continue_drops : DropTree , }
};
}
