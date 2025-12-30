// Generated macro for impl_29 (impl)
macro_rules! Depcrate_analyze_conflictimpl_29 {
() => {
// Module: crate::analyze_conflict
// Provides: {"impl_29"}
// Dependencies: {}
impl AnalyzeConflict { # [doc = " Update structures for a new variable count."] pub fn set_var_count (& mut self , count : usize) { self . var_flags . resize (count , false) ; } # [doc = " The learned clause."] pub fn clause (& self) -> & [Lit] { & self . clause } # [doc = " Long clauses involved in the conflict."] pub fn involved (& self) -> & [ClauseRef] { & self . involved } # [doc = " Hashes of clauses involved in the proof of the learned clause."] # [doc = ""] # [doc = " Hashes are in clause propagation order."] pub fn clause_hashes (& self) -> & [ClauseHash] { & self . clause_hashes } }
};
}
