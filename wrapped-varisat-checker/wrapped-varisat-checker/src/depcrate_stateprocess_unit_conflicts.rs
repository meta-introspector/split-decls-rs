// Generated macro for process_unit_conflicts (function)
macro_rules! Depcrate_stateprocess_unit_conflicts {
() => {
// Module: crate::state
// Provides: {"process_unit_conflicts"}
// Dependencies: {}
# [doc = " Process unit conflicts detected during clause loading."] pub fn process_unit_conflicts < 'a > (mut ctx : partial ! (Context <'a >, mut ProcessingP <'a >, ClausesP , VariablesP) ,) -> Result < () , CheckerError > { let (clauses , mut ctx) = ctx . split_part (ClausesP) ; if let Some (ids) = & clauses . unit_conflict { let clause = & [] ; process_step (ctx . borrow () , & CheckedProofStep :: AtClause { id : clauses . next_clause_id , redundant : false , clause , propagations : ids , } ,) ? ; } Ok (()) }
};
}
