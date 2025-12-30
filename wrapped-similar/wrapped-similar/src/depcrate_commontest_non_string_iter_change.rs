// Generated macro for test_non_string_iter_change (function)
macro_rules! Depcrate_commontest_non_string_iter_change {
() => {
// Module: crate::common
// Provides: {"test_non_string_iter_change"}
// Dependencies: {}
# [test] fn test_non_string_iter_change () { use crate :: ChangeTag ; let old = vec ! [1 , 2 , 3] ; let new = vec ! [1 , 2 , 4] ; let ops = capture_diff_slices (Algorithm :: Myers , & old , & new) ; let changes : Vec < _ > = ops . iter () . flat_map (| x | x . iter_changes (& old , & new)) . map (| x | (x . tag () , x . value ())) . collect () ; assert_eq ! (changes , vec ! [(ChangeTag :: Equal , 1) , (ChangeTag :: Equal , 2) , (ChangeTag :: Delete , 3) , (ChangeTag :: Insert , 4) ,]) ; }
};
}
