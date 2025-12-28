macro_rules! deps {
    () => {
        Pool!();
    };
}

macro_rules! ownedrefs_outlive_pool {
    () => {
        deps!();
        # [test] fn ownedrefs_outlive_pool () { run_model ("ownedrefs_outlive_pool" , | | { let pool = Arc :: new (Pool :: < alloc :: Track < String > > :: new ()) ; let key1 = pool . create_with (| item | item . get_mut () . push_str ("hello")) . expect ("create item 1") ; let key2 = pool . create_with (| item | item . get_mut () . push_str ("goodbye")) . expect ("create item 2") ; let item1_1 = pool . clone () . get_owned (key1) . expect ("get key1") ; let item1_2 = pool . clone () . get_owned (key1) . expect ("get key1 again") ; let item2 = pool . clone () . get_owned (key2) . expect ("get key2") ; drop (pool) ; let t1 = thread :: spawn (move | | { assert_eq ! (item1_1 . get_ref () , & String :: from ("hello")) ; drop (item1_1) ; }) ; let t2 = thread :: spawn (move | | { assert_eq ! (item2 . get_ref () , & String :: from ("goodbye")) ; drop (item2) ; }) ; t1 . join () . unwrap () ; t2 . join () . unwrap () ; assert_eq ! (item1_2 . get_ref () , & String :: from ("hello")) ; }) ; }
    };
}

ownedrefs_outlive_pool!()