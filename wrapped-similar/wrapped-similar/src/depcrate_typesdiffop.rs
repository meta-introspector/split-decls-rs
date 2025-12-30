// Generated macro for DiffOp (enum)
macro_rules! Depcrate_typesDiffOp {
() => {
// Module: crate::types
// Provides: {"DiffOp"}
// Dependencies: {}
# [doc = " Utility enum to capture a diff operation."] # [doc = ""] # [doc = " This is used by [`Capture`](crate::algorithms::Capture)."] # [derive (Debug , PartialEq , Eq , Hash , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize) , serde (rename_all = "snake_case" , tag = "op"))] pub enum DiffOp { # [doc = " A segment is equal (see [`DiffHook::equal`])"] Equal { # [doc = " The starting index in the old sequence."] old_index : usize , # [doc = " The starting index in the new sequence."] new_index : usize , # [doc = " The length of the segment."] len : usize , } , # [doc = " A segment was deleted (see [`DiffHook::delete`])"] Delete { # [doc = " The starting index in the old sequence."] old_index : usize , # [doc = " The length of the old segment."] old_len : usize , # [doc = " The starting index in the new sequence."] new_index : usize , } , # [doc = " A segment was inserted (see [`DiffHook::insert`])"] Insert { # [doc = " The starting index in the old sequence."] old_index : usize , # [doc = " The starting index in the new sequence."] new_index : usize , # [doc = " The length of the new segment."] new_len : usize , } , # [doc = " A segment was replaced (see [`DiffHook::replace`])"] Replace { # [doc = " The starting index in the old sequence."] old_index : usize , # [doc = " The length of the old segment."] old_len : usize , # [doc = " The starting index in the new sequence."] new_index : usize , # [doc = " The length of the new segment."] new_len : usize , } , }
};
}
