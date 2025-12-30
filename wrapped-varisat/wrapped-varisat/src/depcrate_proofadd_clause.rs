// Generated macro for add_clause (function)
macro_rules! Depcrate_proofadd_clause {
() => {
// Module: crate::proof
// Provides: {"add_clause"}
// Dependencies: {}
# [doc = " Call when adding an external clause."] # [doc = ""] # [doc = " This is required for on the fly checking and checking of incremental solving."] # [doc = ""] # [doc = " *Note:* this function expects the clause to use solver var names."] pub fn add_clause < 'a > (mut ctx : partial ! (Context <'a >, mut ProofP <'a >, mut SolverStateP , VariablesP) , clause : & [Lit] ,) { if ctx . part (SolverStateP) . solver_invoked { add_step (ctx . borrow () , true , & ProofStep :: AddClause { clause }) } else { let (variables , mut ctx) = ctx . split_part (VariablesP) ; let proof = ctx . part_mut (ProofP) ; if let Some (checker) = & mut proof . checker { let clause = proof . map_step . map_lits (clause , | var | { variables . global_from_solver () . get (var) . expect ("no existing global var for solver var") }) ; let result = checker . add_clause (clause) ; handle_self_check_result (ctx . borrow () , result) ; } if clause . len () > 1 { ctx . part_mut (ProofP) . clause_count += 1 ; } } }
};
}
