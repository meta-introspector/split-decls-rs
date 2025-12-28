macro_rules! deps {
    () => {
        Pool!();
    };
}

macro_rules! ownedref_send_out_of_local {
    () => {
        deps!();
        # [test] fn ownedref_send_out_of_local () { run_model ("ownedref_send_out_of_local" , | | { let pool = Arc :: new (Pool :: < alloc :: Track < String > > :: new ()) ; let key1 = pool . create_with (| item | item . get_mut () . push_str ("hello")) . expect ("create item 1") ; let key2 = pool . create_with (| item | item . get_mut () . push_str ("goodbye")) . expect ("create item 2") ; let item1 = pool . clone () . get_owned (key1) . expect ("get key1") ; let item2 = pool . clone () . get_owned (key2) . expect ("get key2") ; let pool2 = pool . clone () ; test_dbg ! (pool . clear (key1)) ; let t1 = thread :: spawn (move | | { assert_eq ! (item1 . get_ref () , & String :: from ("hello")) ; drop (item1) ; }) ; let t2 = thread :: spawn (move | | { assert_eq ! (item2 . get_ref () , & String :: from ("goodbye")) ; test_dbg ! (pool2 . clear (key2)) ; drop (item2) ; }) ; t1 . join () . unwrap () ; t2 . join () . unwrap () ; assert ! (pool . get (key1) . is_none ()) ; assert ! (pool . get (key2) . is_none ()) ; }) ; }
    };
}

ownedref_send_out_of_local!()