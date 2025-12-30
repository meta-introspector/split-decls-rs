// Generated macro for shuffle_tests (function)
macro_rules! Depcrate_testsshuffle_tests {
() => {
// Module: crate::tests
// Provides: {"shuffle_tests"}
// Dependencies: {}
# [test] fn shuffle_tests () { let mut opts = TestOpts :: new () ; opts . shuffle = true ; let shuffle_seed = get_shuffle_seed (& opts) . unwrap () ; let left = sample_tests () . into_iter () . enumerate () . map (| (i , e) | (TestId (i) , e)) . collect :: < Vec < _ > > () ; let mut right = sample_tests () . into_iter () . enumerate () . map (| (i , e) | (TestId (i) , e)) . collect :: < Vec < _ > > () ; assert ! (left . iter () . zip (& right) . all (| (a , b) | a . 1 . desc . name == b . 1 . desc . name)) ; helpers :: shuffle :: shuffle_tests (shuffle_seed , right . as_mut_slice ()) ; assert ! (left . iter () . zip (right) . any (| (a , b) | a . 1 . desc . name != b . 1 . desc . name)) ; }
};
}
