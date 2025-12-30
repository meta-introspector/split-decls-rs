// Generated macro for simplify_binary (function)
macro_rules! Depcrate_binarysimplify_binary {
() => {
// Module: crate::binary
// Provides: {"simplify_binary"}
// Dependencies: {}
# [doc = " Remove binary clauses that have an assigned literal."] pub fn simplify_binary < 'a > (mut ctx : partial ! (Context <'a >, mut BinaryClausesP , mut ProofP <'a >, mut SolverStateP , AssignmentP , VariablesP) ,) { let (binary_clauses , mut ctx) = ctx . split_part_mut (BinaryClausesP) ; let (assignment , mut ctx) = ctx . split_part (AssignmentP) ; let mut double_count = 0 ; for (code , implied) in binary_clauses . by_lit . iter_mut () . enumerate () { let lit = Lit :: from_code (code) ; if ! assignment . lit_is_unk (lit) { if ctx . part (ProofP) . is_active () { for & other_lit in implied . iter () { if (! lit) < other_lit { let lits = [! lit , other_lit] ; proof :: add_step (ctx . borrow () , true , & ProofStep :: DeleteClause { clause : & lits [..] , proof : DeleteClauseProof :: Satisfied , } ,) ; } } } implied . clear () ; } else { implied . retain (| & other_lit | { let retain = assignment . lit_is_unk (other_lit) ; if ctx . part (ProofP) . is_active () && ! retain && (! lit) < other_lit { let lits = [! lit , other_lit] ; proof :: add_step (ctx . borrow () , true , & ProofStep :: DeleteClause { clause : & lits [..] , proof : DeleteClauseProof :: Satisfied , } ,) ; } retain }) ; double_count += implied . len () ; } } binary_clauses . count = double_count / 2 ; }
};
}
