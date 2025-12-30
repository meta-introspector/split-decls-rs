// Generated macro for strategy (module)
macro_rules! Depcrate_cnfstrategy {
() => {
// Module: crate::cnf
// Provides: {"strategy"}
// Dependencies: {}
# [cfg (any (test , feature = "proptest-strategies"))] # [doc (hidden)] pub mod strategy { use super :: * ; use proptest :: { collection :: SizeRange , prelude :: * , * } ; use crate :: lit :: strategy :: lit ; pub fn vec_formula (vars : impl Strategy < Value = usize > , clauses : impl Into < SizeRange > , clause_len : impl Into < SizeRange > ,) -> impl Strategy < Value = Vec < Vec < Lit > > > { let clauses = clauses . into () ; let clause_len = clause_len . into () ; vars . prop_ind_flat_map (move | vars | { collection :: vec (collection :: vec (lit (0 .. vars) , clause_len . clone ()) , clauses . clone () ,) }) } pub fn cnf_formula (vars : impl Strategy < Value = usize > , clauses : impl Into < SizeRange > , clause_len : impl Into < SizeRange > ,) -> impl Strategy < Value = CnfFormula > { let clauses = clauses . into () ; let clause_len = clause_len . into () ; let clause_lens = collection :: vec (collection :: vec (Just (()) , clause_len) , clauses) ; (vars , clause_lens) . prop_flat_map (move | (vars , clause_lens) | { let total_lits : usize = clause_lens . iter () . map (| l | l . len ()) . sum () ; collection :: vec (lit (0 .. vars) , total_lits) . prop_map (move | literals | { let mut clause_ranges = Vec :: with_capacity (clause_lens . len ()) ; let mut offset = 0 ; for len in clause_lens . iter () { clause_ranges . push (offset .. offset + len . len ()) ; offset += len . len () ; } CnfFormula { var_count : vars , literals , clause_ranges , } }) . no_shrink () }) } }
};
}
