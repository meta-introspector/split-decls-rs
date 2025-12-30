// Generated macro for make_decision (function)
macro_rules! Depcrate_decisionmake_decision {
() => {
// Module: crate::decision
// Provides: {"make_decision"}
// Dependencies: {}
# [doc = " Make a decision and enqueue it."] # [doc = ""] # [doc = " Returns `false` if no decision was made because all variables are assigned."] pub fn make_decision (mut ctx : partial ! (Context , mut AssignmentP , mut ImplGraphP , mut TrailP , mut VsidsP) ,) -> bool { let (vsids , mut ctx) = ctx . split_part_mut (VsidsP) ; if let Some (decision_var) = vsids . find (| & var | ctx . part (AssignmentP) . var_value (var) . is_none ()) { let decision = decision_var . lit (ctx . part (AssignmentP) . last_var_value (decision_var)) ; ctx . part_mut (TrailP) . new_decision_level () ; enqueue_assignment (ctx . borrow () , decision , Reason :: Unit) ; true } else { false } }
};
}
