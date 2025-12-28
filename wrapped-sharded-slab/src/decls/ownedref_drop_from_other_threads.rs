macro_rules! deps {
    () => {
        Pool!();
    };
}

macro_rules! ownedref_drop_from_other_threads {
    () => {
        deps!();
        # [test] fn ownedref_drop_from_other_threads () { run_model ("ownedref_drop_from_other_threads" , | | { let pool = Arc :: new (Pool :: < alloc :: Track < String > > :: new ()) ; let key1 = pool . create_with (| item | item . get_mut () . push_str ("hello")) . expect ("create item 1") ; let item1 = pool . clone () . get_owned (key1) . expect ("get key1") ; let pool2 = pool . clone () ; let t1 = thread :: spawn (move | | { let pool = pool2 . clone () ; let key2 = pool . create_with (| item | item . get_mut () . push_str ("goodbye")) . expect ("create item 1") ; let item2 = pool . clone () . get_owned (key2) . expect ("get key1") ; let t2 = thread :: spawn (move | | { assert_eq ! (item2 . get_ref () , & String :: from ("goodbye")) ; test_dbg ! (pool2 . clear (key1)) ; drop (item2) }) ; assert_eq ! (item1 . get_ref () , & String :: from ("hello")) ; test_dbg ! (pool . clear (key2)) ; drop (item1) ; (t2 , key2) }) ; let (t2 , key2) = t1 . join () . unwrap () ; test_dbg ! (pool . get (key1)) ; test_dbg ! (pool . get (key2)) ; t2 . join () . unwrap () ; assert ! (pool . get (key1) . is_none ()) ; assert ! (pool . get (key2) . is_none ()) ; }) ; }
    };
}

ownedref_drop_from_other_threads!();