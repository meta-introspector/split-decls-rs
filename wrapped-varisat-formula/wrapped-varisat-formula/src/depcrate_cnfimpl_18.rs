// Generated macro for impl_18 (impl)
macro_rules! Depcrate_cnfimpl_18 {
() => {
// Module: crate::cnf
// Provides: {"impl_18"}
// Dependencies: {}
impl ExtendFormula for CnfFormula { fn add_clause (& mut self , clause : & [Lit]) { let begin = self . literals . len () ; self . literals . extend_from_slice (clause) ; let end = self . literals . len () ; for & lit in self . literals [begin .. end] . iter () { self . var_count = max (lit . index () + 1 , self . var_count) ; } self . clause_ranges . push (begin .. end) ; } fn new_var (& mut self) -> Var { let var = Var :: from_index (self . var_count) ; self . var_count += 1 ; var } }
};
}
