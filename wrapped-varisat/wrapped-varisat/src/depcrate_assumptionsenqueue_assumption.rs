// Generated macro for enqueue_assumption (function)
macro_rules! Depcrate_assumptionsenqueue_assumption {
() => {
// Module: crate::assumptions
// Provides: {"enqueue_assumption"}
// Dependencies: {}
# [doc = " Enqueue another assumption if possible."] # [doc = ""] # [doc = " Returns whether an assumption was enqueued, whether no assumptions are left or whether the"] # [doc = " assumptions result in a conflict."] pub fn enqueue_assumption < 'a > (mut ctx : partial ! (Context <'a >, mut AssignmentP , mut AssumptionsP , mut ImplGraphP , mut ProofP <'a >, mut SolverStateP , mut TmpFlagsP , mut TrailP , ClauseAllocP , VariablesP ,) ,) -> EnqueueAssumption { while let Some (& assumption) = ctx . part (AssumptionsP) . assumptions . get (ctx . part (TrailP) . current_level ()) { match ctx . part (AssignmentP) . lit_value (assumption) { Some (false) => { analyze_assumption_conflict (ctx . borrow () , assumption) ; return EnqueueAssumption :: Conflict ; } Some (true) => { let level = ctx . part (TrailP) . current_level () ; let assumptions = ctx . part_mut (AssumptionsP) ; assumptions . assumptions . swap_remove (level) ; } None => { ctx . part_mut (TrailP) . new_decision_level () ; enqueue_assignment (ctx . borrow () , assumption , Reason :: Unit) ; let (assumptions , ctx) = ctx . split_part_mut (AssumptionsP) ; assumptions . assumption_levels = ctx . part (TrailP) . current_level () ; return EnqueueAssumption :: Enqueued ; } } } EnqueueAssumption :: Done }
};
}
