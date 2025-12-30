// Generated macro for take_remote_and_reuse (function)
macro_rules! Depcrate_tests_loom_slabtake_remote_and_reuse {
() => {
// Module: crate::tests::loom_slab
// Provides: {"take_remote_and_reuse"}
// Dependencies: {}
# [test] fn take_remote_and_reuse () { run_model ("take_remote_and_reuse" , | | { let slab = Arc :: new (Slab :: new_with_config :: < TinyConfig > ()) ; let idx1 = slab . insert (1) . expect ("insert") ; let idx2 = slab . insert (2) . expect ("insert") ; let idx3 = slab . insert (3) . expect ("insert") ; let idx4 = slab . insert (4) . expect ("insert") ; assert_eq ! (slab . get (idx1) . unwrap () , 1 , "slab: {:#?}" , slab) ; assert_eq ! (slab . get (idx2) . unwrap () , 2 , "slab: {:#?}" , slab) ; assert_eq ! (slab . get (idx3) . unwrap () , 3 , "slab: {:#?}" , slab) ; assert_eq ! (slab . get (idx4) . unwrap () , 4 , "slab: {:#?}" , slab) ; let s = slab . clone () ; let t1 = thread :: spawn (move | | { assert_eq ! (s . take (idx1) , Some (1) , "slab: {:#?}" , s) ; }) ; let idx1 = slab . insert (5) . expect ("insert") ; t1 . join () . expect ("thread 1 should not panic") ; assert_eq ! (slab . get (idx1) . unwrap () , 5 , "slab: {:#?}" , slab) ; assert_eq ! (slab . get (idx2) . unwrap () , 2 , "slab: {:#?}" , slab) ; assert_eq ! (slab . get (idx3) . unwrap () , 3 , "slab: {:#?}" , slab) ; assert_eq ! (slab . get (idx4) . unwrap () , 4 , "slab: {:#?}" , slab) ; }) ; }
};
}
