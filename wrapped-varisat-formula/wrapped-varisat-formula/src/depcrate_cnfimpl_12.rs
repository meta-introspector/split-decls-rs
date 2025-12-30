// Generated macro for impl_12 (impl)
macro_rules! Depcrate_cnfimpl_12 {
() => {
// Module: crate::cnf
// Provides: {"impl_12"}
// Dependencies: {}
# [doc = " Convert an iterable of [`Lit`] slices into a CnfFormula"] impl < Clauses , Item > From < Clauses > for CnfFormula where Clauses : IntoIterator < Item = Item > , Item : std :: borrow :: Borrow < [Lit] > , { fn from (clauses : Clauses) -> CnfFormula { let mut cnf_formula = CnfFormula :: new () ; for clause in clauses { cnf_formula . add_clause (clause . borrow ()) ; } cnf_formula } }
};
}
