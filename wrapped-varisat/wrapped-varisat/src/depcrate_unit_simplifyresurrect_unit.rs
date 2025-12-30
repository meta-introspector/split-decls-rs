// Generated macro for resurrect_unit (function)
macro_rules! Depcrate_unit_simplifyresurrect_unit {
() => {
// Module: crate::unit_simplify
// Provides: {"resurrect_unit"}
// Dependencies: {}
# [doc = " Put a removed unit back onto the trail."] pub fn resurrect_unit < 'a > (mut ctx : partial ! (Context <'a >, mut AssignmentP , mut ImplGraphP , mut TrailP) , lit : Lit ,) { if ctx . part (ImplGraphP) . is_removed_unit (lit . var ()) { debug_assert ! (ctx . part (AssignmentP) . lit_is_true (lit)) ; ctx . part_mut (AssignmentP) . unassign_var (lit . var ()) ; enqueue_assignment (ctx . borrow () , lit , Reason :: Unit) ; } }
};
}
