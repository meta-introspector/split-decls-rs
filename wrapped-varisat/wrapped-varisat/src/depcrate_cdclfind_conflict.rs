// Generated macro for find_conflict (function)
macro_rules! Depcrate_cdclfind_conflict {
() => {
// Module: crate::cdcl
// Provides: {"find_conflict"}
// Dependencies: {}
# [doc = " Find a conflict."] # [doc = ""] # [doc = " Returns `Err` if a conflict was found and `Ok` if a satisfying assignment was found instead."] fn find_conflict < 'a > (mut ctx : partial ! (Context <'a >, mut AssignmentP , mut AssumptionsP , mut BinaryClausesP , mut ClauseAllocP , mut ClauseDbP , mut ImplGraphP , mut ProofP <'a >, mut SolverStateP , mut TmpFlagsP , mut TrailP , mut VariablesP , mut VsidsP , mut WatchlistsP ,) ,) -> Result < () , FoundConflict > { loop { let propagation_result = propagate (ctx . borrow ()) ; let new_unit = prove_units (ctx . borrow ()) ; propagation_result ? ; if new_unit { unit_simplify (ctx . borrow ()) ; } match enqueue_assumption (ctx . borrow ()) { EnqueueAssumption :: Enqueued => continue , EnqueueAssumption :: Conflict => return Err (FoundConflict :: Assumption) , EnqueueAssumption :: Done => () , } if ! make_decision (ctx . borrow ()) { return Ok (()) ; } } }
};
}
