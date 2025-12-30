// Generated macro for impl_11 (impl)
macro_rules! Depcrate_cnfimpl_11 {
() => {
// Module: crate::cnf
// Provides: {"impl_11"}
// Dependencies: {}
impl CnfFormula { # [doc = " Create an empty CNF formula."] pub fn new () -> CnfFormula { CnfFormula :: default () } # [doc = " Number of variables in the formula."] # [doc = ""] # [doc = " This also counts missing variables if a variable with a higher index is present."] # [doc = " A vector of this length can be indexed with the variable indices present."] pub fn var_count (& self) -> usize { self . var_count } # [doc = " Increase the number of variables in the formula."] # [doc = ""] # [doc = " If the parameter is less than the current variable count do nothing."] pub fn set_var_count (& mut self , count : usize) { self . var_count = max (self . var_count , count) } # [doc = " Number of clauses in the formula."] pub fn len (& self) -> usize { self . clause_ranges . len () } # [doc = " Whether the set of clauses is empty."] pub fn is_empty (& self) -> bool { self . clause_ranges . is_empty () } # [doc = " Iterator over all clauses."] pub fn iter (& self) -> impl Iterator < Item = & [Lit] > { let literals = & self . literals ; self . clause_ranges . iter () . map (move | range | & literals [range . clone ()]) } }
};
}
