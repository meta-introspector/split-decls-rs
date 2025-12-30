// Generated macro for test_capture_hook_grouping (function)
macro_rules! Depcrate_algorithms_capturetest_capture_hook_grouping {
() => {
// Module: crate::algorithms::capture
// Provides: {"test_capture_hook_grouping"}
// Dependencies: {}
# [test] fn test_capture_hook_grouping () { use crate :: algorithms :: { diff_slices , Algorithm , Replace } ; let rng = (1 .. 100) . collect :: < Vec < _ > > () ; let mut rng_new = rng . clone () ; rng_new [10] = 1000 ; rng_new [13] = 1000 ; rng_new [16] = 1000 ; rng_new [34] = 1000 ; let mut d = Replace :: new (Capture :: new ()) ; diff_slices (Algorithm :: Myers , & mut d , & rng , & rng_new) . unwrap () ; let ops = d . into_inner () . into_grouped_ops (3) ; let tags = ops . iter () . map (| group | group . iter () . map (| x | x . as_tag_tuple ()) . collect :: < Vec < _ > > ()) . collect :: < Vec < _ > > () ; insta :: assert_debug_snapshot ! (ops) ; insta :: assert_debug_snapshot ! (tags) ; }
};
}
