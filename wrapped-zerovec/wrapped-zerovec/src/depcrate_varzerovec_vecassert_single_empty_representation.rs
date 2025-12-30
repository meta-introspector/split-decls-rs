// Generated macro for assert_single_empty_representation (function)
macro_rules! Depcrate_varzerovec_vecassert_single_empty_representation {
() => {
// Module: crate::varzerovec::vec
// Provides: {"assert_single_empty_representation"}
// Dependencies: {}
# [test] fn assert_single_empty_representation () { assert_eq ! (VarZeroVec ::< str >:: new () . as_bytes () , VarZeroVec ::< str >:: from (& [] as & [& str]) . as_bytes ()) ; use crate :: map :: MutableZeroVecLike ; let mut vzv = VarZeroVec :: < str > :: from (& ["hello" , "world"] [..]) ; assert_eq ! (vzv . len () , 2) ; assert ! (! vzv . as_bytes () . is_empty ()) ; vzv . zvl_remove (0) ; assert_eq ! (vzv . len () , 1) ; assert ! (! vzv . as_bytes () . is_empty ()) ; vzv . zvl_remove (0) ; assert_eq ! (vzv . len () , 0) ; assert ! (vzv . as_bytes () . is_empty ()) ; vzv . zvl_insert (0 , "something") ; assert_eq ! (vzv . len () , 1) ; assert ! (! vzv . as_bytes () . is_empty ()) ; }
};
}
