// Generated macro for vacant_entry_2 (function)
macro_rules! Depcrate_tests_loom_slabvacant_entry_2 {
() => {
// Module: crate::tests::loom_slab
// Provides: {"vacant_entry_2"}
// Dependencies: {}
# [test] fn vacant_entry_2 () { run_model ("vacant_entry_2" , | | { let slab = Arc :: new (Slab :: new ()) ; let entry = slab . vacant_entry () . unwrap () ; let key : usize = entry . key () ; let slab2 = slab . clone () ; let slab3 = slab . clone () ; let t1 = thread :: spawn (move | | { test_dbg ! (slab2 . get (key)) ; }) ; entry . insert ("hello world") ; let t2 = thread :: spawn (move | | { test_dbg ! (slab3 . get (key)) ; }) ; t1 . join () . unwrap () ; t2 . join () . unwrap () ; assert_eq ! (slab . get (key) . expect ("get") , "hello world") ; }) ; }
};
}
