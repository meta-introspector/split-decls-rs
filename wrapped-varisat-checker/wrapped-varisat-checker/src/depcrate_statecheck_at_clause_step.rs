// Generated macro for check_at_clause_step (function)
macro_rules! Depcrate_statecheck_at_clause_step {
() => {
// Module: crate::state
// Provides: {"check_at_clause_step"}
// Dependencies: {}
# [doc = " Check an AtClause step"] fn check_at_clause_step < 'a > (mut ctx : partial ! (Context <'a >, mut CheckerStateP , mut ClauseHasherP , mut ClausesP , mut ProcessingP <'a >, mut RupCheckP , mut TmpDataP , mut VariablesP ,) , redundant : bool , clause : & [Lit] , propagation_hashes : & [ClauseHash] ,) -> Result < () , CheckerError > { let mut tmp = replace (& mut ctx . part_mut (TmpDataP) . tmp , vec ! []) ; if copy_canonical (& mut tmp , clause) { return Err (CheckerError :: check_failed (ctx . part (CheckerStateP) . step , format ! ("clause {:?} is a tautology" , tmp) ,)) ; } check_clause_with_hashes (ctx . borrow () , & tmp , & * propagation_hashes) ? ; let (id , added) = store_clause (ctx . borrow () , & tmp , redundant) ; if ! redundant { let state = ctx . part_mut (CheckerStateP) ; state . previous_irred_clause_id = Some (id) ; state . previous_irred_clause_lits . clear () ; state . previous_irred_clause_lits . extend_from_slice (& tmp) ; } match added { StoreClauseResult :: New => { let (rup_check , mut ctx) = ctx . split_part (RupCheckP) ; process_step (ctx . borrow () , & CheckedProofStep :: AtClause { id , redundant , clause : & tmp , propagations : & rup_check . trace_ids , } ,) ? ; } StoreClauseResult :: NewlyIrredundant => { process_step (ctx . borrow () , & CheckedProofStep :: MakeIrredundant { id , clause : & tmp } ,) ? ; } StoreClauseResult :: Duplicate => () , } ctx . part_mut (TmpDataP) . tmp = tmp ; Ok (()) }
};
}
