macro_rules! deps {
    () => {
        Slab!();
    };
}

macro_rules! owned_entry_drop_from_other_threads {
    () => {
        deps!();
        # [test] fn owned_entry_drop_from_other_threads () { run_model ("owned_entry_drop_from_other_threads" , | | { let slab = Arc :: new (Slab :: < alloc :: Track < String > > :: new ()) ; let key1 = slab . insert (alloc :: Track :: new (String :: from ("hello"))) . expect ("insert item 1") ; let item1 = slab . clone () . get_owned (key1) . expect ("get key1") ; let slab2 = slab . clone () ; let t1 = thread :: spawn (move | | { let slab = slab2 . clone () ; let key2 = slab . insert (alloc :: Track :: new (String :: from ("goodbye"))) . expect ("insert item 1") ; let item2 = slab . clone () . get_owned (key2) . expect ("get key1") ; let t2 = thread :: spawn (move | | { assert_eq ! (item2 . get_ref () , & String :: from ("goodbye")) ; test_dbg ! (slab2 . remove (key1)) ; drop (item2) }) ; assert_eq ! (item1 . get_ref () , & String :: from ("hello")) ; test_dbg ! (slab . remove (key2)) ; drop (item1) ; (t2 , key2) }) ; let (t2 , key2) = t1 . join () . unwrap () ; test_dbg ! (slab . get (key1)) ; test_dbg ! (slab . get (key2)) ; t2 . join () . unwrap () ; assert ! (slab . get (key1) . is_none ()) ; assert ! (slab . get (key2) . is_none ()) ; }) ; }
    };
}

owned_entry_drop_from_other_threads!()