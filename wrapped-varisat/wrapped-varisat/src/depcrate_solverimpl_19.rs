// Generated macro for impl_19 (impl)
macro_rules! Depcrate_solverimpl_19 {
() => {
// Module: crate::solver
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'a > ExtendFormula for Solver < 'a > { # [doc = " Add a clause to the solver."] fn add_clause (& mut self , clause : & [Lit]) { let mut ctx = self . ctx . into_partial_ref_mut () ; load_clause (ctx . borrow () , clause) ; } # [doc = " Add a new variable to the solver."] fn new_var (& mut self) -> Var { self . ctx . solver_state . formula_is_empty = false ; let mut ctx = self . ctx . into_partial_ref_mut () ; variables :: new_user_var (ctx . borrow ()) } }
};
}
