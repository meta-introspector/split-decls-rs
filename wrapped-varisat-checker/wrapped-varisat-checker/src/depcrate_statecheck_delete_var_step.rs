// Generated macro for check_delete_var_step (function)
macro_rules! Depcrate_statecheck_delete_var_step {
() => {
// Module: crate::state
// Provides: {"check_delete_var_step"}
// Dependencies: {}
# [doc = " Check a DeleteVar step"] fn check_delete_var_step < 'a > (mut ctx : partial ! (Context <'a >, mut ClausesP , mut ProcessingP <'a >, mut VariablesP , CheckerStateP ,) , var : Var ,) -> Result < () , CheckerError > { ensure_var (ctx . borrow () , var) ; if let Some (user_var) = ctx . part (VariablesP) . var_data [var . index ()] . user_var { return Err (CheckerError :: check_failed (ctx . part (CheckerStateP) . step , format ! ("deleted variable {:?} corresponds to user variable {:?}" , var , user_var) ,)) ; } for & polarity in & [false , true] { if ctx . part (VariablesP) . lit_data [var . lit (polarity) . code ()] . clause_count > 0 { return Err (CheckerError :: check_failed (ctx . part (CheckerStateP) . step , format ! ("deleted variable {:?} still has clauses" , var) ,)) ; } } if let Some (unit_clause) = ctx . part (ClausesP) . unit_clauses [var . index ()] { let clause = [var . lit (unit_clause . value)] ; let id = match unit_clause . id { UnitId :: Global (id) => id , _ => unreachable ! () , } ; process_step (ctx . borrow () , & CheckedProofStep :: DeleteRatClause { id , keep_as_redundant : false , clause : & clause [..] , pivot : clause [0] , propagations : & ResolutionPropagations { } , } ,) ? ; ctx . part_mut (ClausesP) . unit_clauses [var . index ()] = None ; } ctx . part_mut (VariablesP) . var_data [var . index ()] = VarData :: default () ; Ok (()) }
};
}
