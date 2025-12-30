// Generated macro for racy_take_local (function)
macro_rules! Depcrate_tests_loom_slabracy_take_local {
() => {
// Module: crate::tests::loom_slab
// Provides: {"racy_take_local"}
// Dependencies: {}
# [test] fn racy_take_local () { run_model ("racy_take_local" , | | { let slab = Arc :: new (Slab :: new ()) ; let idx = slab . insert (1) . expect ("insert") ; assert_eq ! (slab . get (idx) . unwrap () , 1) ; let s = slab . clone () ; let t2 = thread :: spawn (move | | s . take (idx)) ; let r1 = slab . take (idx) ; let r2 = t2 . join () . expect ("thread 2 should not panic") ; assert ! (r1 . is_none () || r2 . is_none () , "both threads should not have removed the value") ; assert ! (r1 . or (r2) . is_some () , "one thread should have removed the value") ; assert ! (slab . get (idx) . is_none ()) ; }) ; }
};
}
