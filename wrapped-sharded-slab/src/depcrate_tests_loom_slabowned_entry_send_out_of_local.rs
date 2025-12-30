// Generated macro for owned_entry_send_out_of_local (function)
macro_rules! Depcrate_tests_loom_slabowned_entry_send_out_of_local {
() => {
// Module: crate::tests::loom_slab
// Provides: {"owned_entry_send_out_of_local"}
// Dependencies: {}
# [test] fn owned_entry_send_out_of_local () { run_model ("owned_entry_send_out_of_local" , | | { let slab = Arc :: new (Slab :: < alloc :: Track < String > > :: new ()) ; let key1 = slab . insert (alloc :: Track :: new (String :: from ("hello"))) . expect ("insert item 1") ; let key2 = slab . insert (alloc :: Track :: new (String :: from ("goodbye"))) . expect ("insert item 2") ; let item1 = slab . clone () . get_owned (key1) . expect ("get key1") ; let item2 = slab . clone () . get_owned (key2) . expect ("get key2") ; let slab2 = slab . clone () ; test_dbg ! (slab . remove (key1)) ; let t1 = thread :: spawn (move | | { assert_eq ! (item1 . get_ref () , & String :: from ("hello")) ; drop (item1) ; }) ; let t2 = thread :: spawn (move | | { assert_eq ! (item2 . get_ref () , & String :: from ("goodbye")) ; test_dbg ! (slab2 . remove (key2)) ; drop (item2) ; }) ; t1 . join () . unwrap () ; t2 . join () . unwrap () ; assert ! (slab . get (key1) . is_none ()) ; assert ! (slab . get (key2) . is_none ()) ; }) ; }
};
}
