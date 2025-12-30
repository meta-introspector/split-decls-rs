// Generated macro for test_pairule_validate (function)
macro_rules! Depcrate_ule_tupletest_pairule_validate {
() => {
// Module: crate::ule::tuple
// Provides: {"test_pairule_validate"}
// Dependencies: {}
# [test] fn test_pairule_validate () { use crate :: ZeroVec ; let vec : Vec < (u32 , char) > = vec ! [(1 , 'a') , (1234901 , '啊') , (100 , 'अ')] ; let zerovec : ZeroVec < (u32 , char) > = vec . iter () . copied () . collect () ; let bytes = zerovec . as_bytes () ; let zerovec2 = ZeroVec :: parse_bytes (bytes) . unwrap () ; assert_eq ! (zerovec , zerovec2) ; let zerovec3 = ZeroVec :: < (char , u32) > :: parse_bytes (bytes) ; assert ! (zerovec3 . is_err ()) ; }
};
}
