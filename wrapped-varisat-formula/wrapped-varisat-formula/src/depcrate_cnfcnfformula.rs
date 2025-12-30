// Generated macro for CnfFormula (struct)
macro_rules! Depcrate_cnfCnfFormula {
() => {
// Module: crate::cnf
// Provides: {"CnfFormula"}
// Dependencies: {}
# [doc = " A formula in conjunctive normal form (CNF)."] # [doc = ""] # [doc = " Equivalent to Vec<Vec<Lit>> but more efficient as it uses a single buffer for all literals."] # [derive (Clone , Default , Eq)] pub struct CnfFormula { var_count : usize , literals : Vec < Lit > , clause_ranges : Vec < Range < usize > > , }
};
}
