// Generated macro for vacant_entry_remove (function)
macro_rules! Depcrate_tests_loom_slabvacant_entry_remove {
() => {
// Module: crate::tests::loom_slab
// Provides: {"vacant_entry_remove"}
// Dependencies: {}
# [test] fn vacant_entry_remove () { run_model ("vacant_entry_remove" , | | { let slab = Arc :: new (Slab :: new ()) ; let entry = slab . vacant_entry () . unwrap () ; let key : usize = entry . key () ; let slab2 = slab . clone () ; let t1 = thread :: spawn (move | | { assert ! (! slab2 . remove (key)) ; }) ; t1 . join () . unwrap () ; entry . insert ("hello world") ; assert_eq ! (slab . get (key) . expect ("get") , "hello world") ; }) ; }
};
}
