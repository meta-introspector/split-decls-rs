// Generated macro for custom_page_sz (function)
macro_rules! Depcrate_tests_loom_slabcustom_page_sz {
() => {
// Module: crate::tests::loom_slab
// Provides: {"custom_page_sz"}
// Dependencies: {}
# [test] fn custom_page_sz () { let mut model = loom :: model :: Builder :: new () ; model . max_branches = 100000 ; model . check (| | { let slab = Slab :: < usize > :: new_with_config :: < TinyConfig > () ; for i in 0 .. 1024usize { test_println ! ("{}" , i) ; let k = slab . insert (i) . expect ("insert") ; let v = slab . get (k) . expect ("get") ; assert_eq ! (v , i , "slab: {:#?}" , slab) ; } }) ; }
};
}
