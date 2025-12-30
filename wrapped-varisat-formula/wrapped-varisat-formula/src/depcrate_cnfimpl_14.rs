// Generated macro for impl_14 (impl)
macro_rules! Depcrate_cnfimpl_14 {
() => {
// Module: crate::cnf
// Provides: {"impl_14"}
// Dependencies: {}
impl PartialEq for CnfFormula { fn eq (& self , other : & CnfFormula) -> bool { self . var_count () == other . var_count () && self . clause_ranges . len () == other . clause_ranges . len () && self . clause_ranges . iter () . zip (other . clause_ranges . iter ()) . all (| (range_a , range_b) | { self . literals [range_a . clone ()] == other . literals [range_b . clone ()] }) } }
};
}
