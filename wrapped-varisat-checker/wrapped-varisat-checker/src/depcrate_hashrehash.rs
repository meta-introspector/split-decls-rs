// Generated macro for rehash (function)
macro_rules! Depcrate_hashrehash {
() => {
// Module: crate::hash
// Provides: {"rehash"}
// Dependencies: {}
# [doc = " Recompute all clause hashes if necessary"] pub fn rehash (mut ctx : partial ! (Context , mut ClauseHasherP , mut ClausesP)) { let (hasher , mut ctx) = ctx . split_part_mut (ClauseHasherP) ; let clauses = ctx . part_mut (ClausesP) ; for (global , solver) in hasher . buffered_solver_var_names . drain (..) { if let Some (solver) = solver { hasher . solver_var_names . insert (global , solver) ; } else { hasher . solver_var_names . remove (& global) ; } } hasher . rename_in_buffered_solver_var_names = false ; let mut old_clauses = take (& mut clauses . clauses) ; for (_ , mut candidates) in old_clauses . drain () { for clause in candidates . drain (..) { let hash = hasher . clause_hash (clause . lits . slice (& clauses . literal_buffer)) ; let candidates = clauses . clauses . entry (hash) . or_default () ; candidates . push (clause) ; } } }
};
}
