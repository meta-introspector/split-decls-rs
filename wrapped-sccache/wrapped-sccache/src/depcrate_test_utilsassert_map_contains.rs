// Generated macro for assert_map_contains (macro)
macro_rules! Depcrate_test_utilsassert_map_contains {
() => {
// Module: crate::test::utils
// Provides: {"assert_map_contains"}
// Dependencies: {}
# [doc = " Assert that `map` contains all of the (`key`, `val`) pairs specified and only those keys."] macro_rules ! assert_map_contains { ($ map : expr , $ (($ key : expr , $ val : expr)) ,*) => { let mut nelems = 0 ; $ (nelems += 1 ; match $ map . get (&$ key) { Some (& ref v) => assert_eq ! ($ val , * v , "{} key `{:?}` doesn't match expected! (expected `{:?}` != actual `{:?}`)" , stringify ! ($ map) , $ key , $ val , v) , None => panic ! ("{} missing key `{:?}`" , stringify ! ($ map) , $ key) , }) * assert_eq ! (nelems , $ map . len () , "{} contains {} elements, expected {}" , stringify ! ($ map) , $ map . len () , nelems) ; } }
};
}
