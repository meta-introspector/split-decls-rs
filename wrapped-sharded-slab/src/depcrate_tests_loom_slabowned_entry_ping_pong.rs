// Generated macro for owned_entry_ping_pong (function)
macro_rules! Depcrate_tests_loom_slabowned_entry_ping_pong {
() => {
// Module: crate::tests::loom_slab
// Provides: {"owned_entry_ping_pong"}
// Dependencies: {}
# [test] fn owned_entry_ping_pong () { run_model ("owned_entry_ping_pong" , | | { let slab = Arc :: new (Slab :: < alloc :: Track < String > > :: new ()) ; let key1 = slab . insert (alloc :: Track :: new (String :: from ("hello"))) . expect ("insert item 1") ; let key2 = slab . insert (alloc :: Track :: new (String :: from ("world"))) . expect ("insert item 2") ; let item1 = slab . clone () . get_owned (key1) . expect ("get key1") ; let slab2 = slab . clone () ; let slab3 = slab . clone () ; let t1 = thread :: spawn (move | | { assert_eq ! (item1 . get_ref () , & String :: from ("hello")) ; slab2 . remove (key1) ; item1 }) ; let t2 = thread :: spawn (move | | { let item2 = slab3 . clone () . get_owned (key2) . unwrap () ; assert_eq ! (item2 . get_ref () , & String :: from ("world")) ; slab3 . remove (key1) ; item2 }) ; let item1 = t1 . join () . unwrap () ; let item2 = t2 . join () . unwrap () ; assert_eq ! (item1 . get_ref () , & String :: from ("hello")) ; assert_eq ! (item2 . get_ref () , & String :: from ("world")) ; }) ; }
};
}
