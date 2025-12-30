// Generated macro for store_unit_clause (function)
macro_rules! Depcrate_clausesstore_unit_clause {
() => {
// Module: crate::clauses
// Provides: {"store_unit_clause"}
// Dependencies: {}
# [doc = " Adds a unit clause to the checker data structures."] # [doc = ""] # [doc = " Returns the id of the added clause and a boolean that is true if the clause wasn't already"] # [doc = " present."] pub fn store_unit_clause (mut ctx : partial ! (Context , mut CheckerStateP , mut ClausesP) , lit : Lit ,) -> (u64 , StoreClauseResult) { match ctx . part (ClausesP) . lit_value (lit) { Some ((true , UnitClause { id : UnitId :: Global (id) , .. } ,)) => (id , StoreClauseResult :: Duplicate) , Some ((false , UnitClause { id : UnitId :: Global (conflicting_id) , .. } ,)) => { ctx . part_mut (CheckerStateP) . unsat = true ; let id = ctx . part (ClausesP) . next_clause_id ; ctx . part_mut (ClausesP) . unit_conflict = Some ([conflicting_id , id]) ; ctx . part_mut (ClausesP) . next_clause_id += 1 ; (id , StoreClauseResult :: New) } Some (_) => unreachable ! () , None => { let id = ctx . part (ClausesP) . next_clause_id ; ctx . part_mut (ClausesP) . unit_clauses [lit . index ()] = Some (UnitClause { value : lit . is_positive () , id : UnitId :: Global (id) , }) ; ctx . part_mut (ClausesP) . next_clause_id += 1 ; (id , StoreClauseResult :: New) } } }
};
}
