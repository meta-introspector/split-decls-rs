// Generated macro for check_failed_assumptions_step (function)
macro_rules! Depcrate_statecheck_failed_assumptions_step {
() => {
// Module: crate::state
// Provides: {"check_failed_assumptions_step"}
// Dependencies: {}
# [doc = " Check a FailedAssumptions step"] fn check_failed_assumptions_step < 'a > (mut ctx : partial ! (Context <'a >, mut ClauseHasherP , mut ClausesP , mut ProcessingP <'a >, mut RupCheckP , mut TmpDataP , mut VariablesP , CheckerStateP ,) , failed_core : & [Lit] , propagation_hashes : & [ClauseHash] ,) -> Result < () , CheckerError > { let mut tmp = replace (& mut ctx . part_mut (TmpDataP) . tmp , vec ! []) ; let direct_conflict = copy_canonical (& mut tmp , failed_core) ; if ! is_subset (& tmp , & ctx . part (CheckerStateP) . assumptions , false) { return Err (CheckerError :: check_failed (ctx . part (CheckerStateP) . step , "failed core contains non-assumed variables" . to_string () ,)) ; } if direct_conflict { ctx . part_mut (RupCheckP) . trace_ids . clear () ; } else { for lit in tmp . iter_mut () { * lit = ! * lit ; } check_clause_with_hashes (ctx . borrow () , & tmp , propagation_hashes) ? ; for lit in tmp . iter_mut () { * lit = ! * lit ; } } let (rup_check , mut ctx) = ctx . split_part (RupCheckP) ; process_step (ctx . borrow () , & CheckedProofStep :: FailedAssumptions { failed_core : & tmp , propagations : & rup_check . trace_ids , } ,) ? ; ctx . part_mut (TmpDataP) . tmp = tmp ; Ok (()) }
};
}
