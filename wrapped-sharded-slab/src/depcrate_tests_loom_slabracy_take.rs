// Generated macro for racy_take (function)
macro_rules! Depcrate_tests_loom_slabracy_take {
() => {
// Module: crate::tests::loom_slab
// Provides: {"racy_take"}
// Dependencies: {}
# [test] fn racy_take () { run_model ("racy_take" , | | { let slab = Arc :: new (Slab :: new ()) ; let idx = slab . insert (1) . expect ("insert") ; assert_eq ! (slab . get (idx) . unwrap () , 1) ; let s1 = slab . clone () ; let s2 = slab . clone () ; let t1 = thread :: spawn (move | | s1 . take (idx)) ; let t2 = thread :: spawn (move | | s2 . take (idx)) ; let r1 = t1 . join () . expect ("thread 1 should not panic") ; let r2 = t2 . join () . expect ("thread 2 should not panic") ; assert ! (r1 . is_none () || r2 . is_none () , "both threads should not have removed the value") ; assert_eq ! (r1 . or (r2) , Some (1) , "one thread should have removed the value") ; assert ! (slab . get (idx) . is_none ()) ; }) ; }
};
}
