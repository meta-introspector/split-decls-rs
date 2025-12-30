// Generated macro for set_var_count (function)
macro_rules! Depcrate_contextset_var_count {
() => {
// Module: crate::context
// Provides: {"set_var_count"}
// Dependencies: {}
# [doc = " Update structures for a new variable count."] pub fn set_var_count (mut ctx : partial ! (Context , mut AnalyzeConflictP , mut AssignmentP , mut BinaryClausesP , mut ImplGraphP , mut TmpFlagsP , mut VsidsP , mut WatchlistsP ,) , count : usize ,) { ctx . part_mut (AnalyzeConflictP) . set_var_count (count) ; ctx . part_mut (AssignmentP) . set_var_count (count) ; ctx . part_mut (BinaryClausesP) . set_var_count (count) ; ctx . part_mut (ImplGraphP) . set_var_count (count) ; ctx . part_mut (TmpFlagsP) . set_var_count (count) ; ctx . part_mut (VsidsP) . set_var_count (count) ; ctx . part_mut (WatchlistsP) . set_var_count (count) ; }
};
}
