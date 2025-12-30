// Generated macro for remove_remote (function)
macro_rules! Depcrate_tests_loom_slabremove_remote {
() => {
// Module: crate::tests::loom_slab
// Provides: {"remove_remote"}
// Dependencies: {}
# [test] fn remove_remote () { run_model ("remove_remote" , | | { let slab = Arc :: new (Slab :: new_with_config :: < TinyConfig > ()) ; let slab2 = slab . clone () ; let (dropped , item) = AssertDropped :: new (1) ; let idx = slab . insert (item) . expect ("insert") ; assert ! (slab . remove (idx)) ; let t1 = thread :: spawn (move | | { let g = slab2 . get (idx) ; drop (g) ; }) ; t1 . join () . expect ("thread 1 should not panic") ; assert ! (slab . get (idx) . is_none ()) ; dropped . assert_dropped () ; }) ; }
};
}
