// Generated macro for vacant_entry (function)
macro_rules! Depcrate_tests_loom_slabvacant_entry {
() => {
// Module: crate::tests::loom_slab
// Provides: {"vacant_entry"}
// Dependencies: {}
# [test] fn vacant_entry () { run_model ("vacant_entry" , | | { let slab = Arc :: new (Slab :: new ()) ; let entry = slab . vacant_entry () . unwrap () ; let key : usize = entry . key () ; let slab2 = slab . clone () ; let t1 = thread :: spawn (move | | { test_dbg ! (slab2 . get (key)) ; }) ; entry . insert ("hello world") ; t1 . join () . unwrap () ; assert_eq ! (slab . get (key) . expect ("get") , "hello world") ; }) ; }
};
}
