// Generated macro for collect_garbage (function)
macro_rules! Depcrate_clausescollect_garbage {
() => {
// Module: crate::clauses
// Provides: {"collect_garbage"}
// Dependencies: {}
# [doc = " Perform a garbage collection if required"] fn collect_garbage (mut ctx : partial ! (Context , mut ClausesP)) { let clauses = ctx . part_mut (ClausesP) ; if clauses . garbage_size * 2 <= clauses . literal_buffer . len () { return ; } let mut new_buffer = vec ! [] ; new_buffer . reserve (clauses . literal_buffer . len ()) ; for (_ , candidates) in clauses . clauses . iter_mut () { for clause in candidates . iter_mut () { let new_lits = ClauseLits :: new (clause . lits . slice (& clauses . literal_buffer) , & mut new_buffer) ; clause . lits = new_lits ; } } clauses . literal_buffer = new_buffer ; clauses . garbage_size = 0 ; }
};
}
