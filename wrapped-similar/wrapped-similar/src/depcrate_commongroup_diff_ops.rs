// Generated macro for group_diff_ops (function)
macro_rules! Depcrate_commongroup_diff_ops {
() => {
// Module: crate::common
// Provides: {"group_diff_ops"}
// Dependencies: {}
# [doc = " Isolate change clusters by eliminating ranges with no changes."] # [doc = ""] # [doc = " This will leave holes behind in long periods of equal ranges so that"] # [doc = " you can build things like unified diffs."] pub fn group_diff_ops (mut ops : Vec < DiffOp > , n : usize) -> Vec < Vec < DiffOp > > { if ops . is_empty () { return vec ! [] ; } let mut pending_group = Vec :: new () ; let mut rv = Vec :: new () ; if let Some (DiffOp :: Equal { old_index , new_index , len , }) = ops . first_mut () { let offset = (* len) . saturating_sub (n) ; * old_index += offset ; * new_index += offset ; * len -= offset ; } if let Some (DiffOp :: Equal { len , .. }) = ops . last_mut () { * len -= (* len) . saturating_sub (n) ; } for op in ops . into_iter () { if let DiffOp :: Equal { old_index , new_index , len , } = op { if len > n * 2 { pending_group . push (DiffOp :: Equal { old_index , new_index , len : n , }) ; rv . push (pending_group) ; let offset = len . saturating_sub (n) ; pending_group = vec ! [DiffOp :: Equal { old_index : old_index + offset , new_index : new_index + offset , len : len - offset , }] ; continue ; } } pending_group . push (op) ; } match & pending_group [..] { & [] | & [DiffOp :: Equal { .. }] => { } _ => rv . push (pending_group) , } rv }
};
}
