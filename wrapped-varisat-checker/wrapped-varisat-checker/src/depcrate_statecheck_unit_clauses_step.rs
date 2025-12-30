// Generated macro for check_unit_clauses_step (function)
macro_rules! Depcrate_statecheck_unit_clauses_step {
() => {
// Module: crate::state
// Provides: {"check_unit_clauses_step"}
// Dependencies: {}
# [doc = " Check a UnitClauses step"] fn check_unit_clauses_step < 'a > (mut ctx : partial ! (Context <'a >, mut CheckerStateP , mut ClauseHasherP , mut ClausesP , mut ProcessingP <'a >, mut RupCheckP , mut VariablesP ,) , units : & [(Lit , ClauseHash)] ,) -> Result < () , CheckerError > { for & (lit , hash) in units . iter () { ensure_var (ctx . borrow () , lit . var ()) ; let clause = [lit] ; let propagation_hashes = [hash] ; check_clause_with_hashes (ctx . borrow () , & clause [..] , & propagation_hashes [..]) ? ; let (id , added) = store_unit_clause (ctx . borrow () , lit) ; match added { StoreClauseResult :: New => { let (rup_check , mut ctx) = ctx . split_part (RupCheckP) ; process_step (ctx . borrow () , & CheckedProofStep :: AtClause { id , redundant : false , clause : & clause , propagations : & rup_check . trace_ids , } ,) ? ; } StoreClauseResult :: Duplicate => () , StoreClauseResult :: NewlyIrredundant => unreachable ! () , } } Ok (()) }
};
}
