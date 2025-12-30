// Generated macro for impl_90 (impl)
macro_rules! Depcrate_stateimpl_90 {
() => {
// Module: crate::state
// Provides: {"impl_90"}
// Dependencies: {}
impl CheckerState { # [doc = " Check whether a given clause is subsumed by the last added irredundant clause."] # [doc = ""] # [doc = " `lits` must be sorted and free of duplicates."] fn subsumed_by_previous_irred_clause (& self , lits : & [Lit]) -> bool { if self . previous_irred_clause_id . is_none () { return false ; } is_subset (& self . previous_irred_clause_lits [..] , lits , true) } }
};
}
