// Generated macro for order_depends_on_more_than_seed (function)
macro_rules! Depcrate_testsorder_depends_on_more_than_seed {
() => {
// Module: crate::tests
// Provides: {"order_depends_on_more_than_seed"}
// Dependencies: {}
# [test] fn order_depends_on_more_than_seed () { let mut opts = TestOpts :: new () ; opts . shuffle = true ; let shuffle_seed = get_shuffle_seed (& opts) . unwrap () ; let mut left_tests = sample_tests () ; let mut right_tests = sample_tests () ; left_tests . pop () ; right_tests . remove (0) ; let mut left = left_tests . into_iter () . enumerate () . map (| (i , e) | (TestId (i) , e)) . collect :: < Vec < _ > > () ; let mut right = right_tests . into_iter () . enumerate () . map (| (i , e) | (TestId (i) , e)) . collect :: < Vec < _ > > () ; assert_eq ! (left . len () , right . len ()) ; assert ! (left . iter () . zip (& right) . all (| (a , b) | a . 0 == b . 0)) ; helpers :: shuffle :: shuffle_tests (shuffle_seed , left . as_mut_slice ()) ; helpers :: shuffle :: shuffle_tests (shuffle_seed , right . as_mut_slice ()) ; assert ! (left . iter () . zip (right) . any (| (a , b) | a . 0 != b . 0)) ; }
};
}
