// Generated macro for shuffle_tests_with_seed (function)
macro_rules! Depcrate_stats_testsshuffle_tests_with_seed {
() => {
// Module: crate::stats::tests
// Provides: {"shuffle_tests_with_seed"}
// Dependencies: {}
# [test] fn shuffle_tests_with_seed () { let mut opts = TestOpts :: new () ; opts . shuffle = true ; let shuffle_seed = get_shuffle_seed (& opts) . unwrap () ; let mut left = sample_tests () . into_iter () . enumerate () . map (| (i , e) | (TestId (i) , e)) . collect :: < Vec < _ > > () ; let mut right = sample_tests () . into_iter () . enumerate () . map (| (i , e) | (TestId (i) , e)) . collect :: < Vec < _ > > () ; helpers :: shuffle :: shuffle_tests (shuffle_seed , left . as_mut_slice ()) ; helpers :: shuffle :: shuffle_tests (shuffle_seed , right . as_mut_slice ()) ; assert ! (left . iter () . zip (right) . all (| (a , b) | a . 1 . desc . name == b . 1 . desc . name)) ; }
};
}
