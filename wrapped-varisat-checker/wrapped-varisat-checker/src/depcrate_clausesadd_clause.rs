// Generated macro for add_clause (function)
macro_rules! Depcrate_clausesadd_clause {
() => {
// Module: crate::clauses
// Provides: {"add_clause"}
// Dependencies: {}
# [doc = " Adds a clause to the checker."] pub fn add_clause < 'a > (mut ctx : partial ! (Context <'a >, mut ClausesP , mut CheckerStateP , mut ProcessingP <'a >, mut TmpDataP , mut VariablesP , ClauseHasherP) , clause : & [Lit] ,) -> Result < () , CheckerError > { if ctx . part (CheckerStateP) . unsat { return Ok (()) ; } let (tmp_data , mut ctx) = ctx . split_part_mut (TmpDataP) ; if copy_canonical (& mut tmp_data . tmp , clause) { let (clauses , mut ctx) = ctx . split_part_mut (ClausesP) ; process_step (ctx . borrow () , & CheckedProofStep :: TautologicalClause { id : clauses . next_clause_id , clause : & tmp_data . tmp , } ,) ? ; clauses . next_clause_id += 1 ; return Ok (()) ; } for & lit in tmp_data . tmp . iter () { ensure_sampling_var (ctx . borrow () , lit . var ()) ? ; } let (id , added) = store_clause (ctx . borrow () , & tmp_data . tmp , false) ; let (clauses , mut ctx) = ctx . split_part_mut (ClausesP) ; match added { StoreClauseResult :: New => { process_step (ctx . borrow () , & CheckedProofStep :: AddClause { id , clause : & tmp_data . tmp , } ,) ? ; } StoreClauseResult :: NewlyIrredundant | StoreClauseResult :: Duplicate => { if let StoreClauseResult :: NewlyIrredundant = added { process_step (ctx . borrow () , & CheckedProofStep :: MakeIrredundant { id , clause : & tmp_data . tmp , } ,) ? ; } process_step (ctx . borrow () , & CheckedProofStep :: DuplicatedClause { id : clauses . next_clause_id , same_as_id : id , clause : & tmp_data . tmp , } ,) ? ; clauses . next_clause_id += 1 ; } } Ok (()) }
};
}
