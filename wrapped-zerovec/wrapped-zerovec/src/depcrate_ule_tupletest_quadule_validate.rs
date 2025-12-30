// Generated macro for test_quadule_validate (function)
macro_rules! Depcrate_ule_tupletest_quadule_validate {
() => {
// Module: crate::ule::tuple
// Provides: {"test_quadule_validate"}
// Dependencies: {}
# [test] fn test_quadule_validate () { use crate :: ZeroVec ; let vec : Vec < (u32 , char , i8 , u16) > = vec ! [(1 , 'a' , - 5 , 3) , (1234901 , '啊' , 3 , 11) , (100 , 'अ' , - 127 , 0)] ; let zerovec : ZeroVec < (u32 , char , i8 , u16) > = vec . iter () . copied () . collect () ; let bytes = zerovec . as_bytes () ; let zerovec2 = ZeroVec :: parse_bytes (bytes) . unwrap () ; assert_eq ! (zerovec , zerovec2) ; let zerovec3 = ZeroVec :: < (char , i8 , u16 , u32) > :: parse_bytes (bytes) ; assert ! (zerovec3 . is_err ()) ; }
};
}
