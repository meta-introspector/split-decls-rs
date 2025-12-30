// Generated macro for tests (module)
macro_rules! Depcrate_cnftests {
() => {
// Module: crate::cnf
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: { strategy :: * , * } ; use proptest :: * ; # [test] fn new_vars () { let mut formula = CnfFormula :: new () ; let (x , y , z) = formula . new_vars () ; assert_ne ! (x , y) ; assert_ne ! (y , z) ; assert_ne ! (x , y) ; assert_eq ! (formula . var_count () , 3) ; } # [test] fn simple_roundtrip () { let input = cnf ! [1 , 2 , 3 ; - 1 , - 2 ; 7 , 2 ; ; 4 , 5 ;] ; let formula = CnfFormula :: from (input . iter () . cloned ()) ; for (clause , & ref_clause) in formula . iter () . zip (input . iter ()) { assert_eq ! (clause , ref_clause) ; } assert_eq ! (formula . var_count () , 7) ; } proptest ! { # [test] fn roundtrip_from_vec (input in vec_formula (1 .. 200usize , 0 .. 1000 , 0 .. 10)) { let formula = CnfFormula :: from (input . clone ()) ; for (clause , ref_clause) in formula . iter () . zip (input . iter ()) { prop_assert_eq ! (clause , & ref_clause [..]) ; } let var_count = input . iter () . flat_map (| clause | clause . iter () . map (| lit | lit . index () + 1)) . max () . unwrap_or (0) ; prop_assert_eq ! (formula . var_count () , var_count) ; } # [test] fn roundtrip_from_cnf (input in cnf_formula (1 .. 100usize , 0 .. 1000 , 0 .. 10)) { let roundtrip = CnfFormula :: from (input . iter ()) ; for (clause_a , clause_b) in input . iter () . zip (roundtrip . iter ()) { prop_assert_eq ! (clause_a , clause_b) ; } prop_assert ! (roundtrip . var_count () <= input . var_count ()) ; if roundtrip . var_count () == input . var_count () { prop_assert_eq ! (roundtrip , input) ; } } } }
};
}
