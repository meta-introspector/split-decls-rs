macro_rules! deps {
    () => {
        Pool!();
    };
}

macro_rules! ownedref_ping_pong {
    () => {
        deps!();
        # [test] fn ownedref_ping_pong () { run_model ("ownedref_ping_pong" , | | { let pool = Arc :: new (Pool :: < alloc :: Track < String > > :: new ()) ; let key1 = pool . create_with (| item | item . get_mut () . push_str ("hello")) . expect ("create item 1") ; let key2 = pool . create_with (| item | item . get_mut () . push_str ("world")) . expect ("create item 2") ; let item1 = pool . clone () . get_owned (key1) . expect ("get key1") ; let pool2 = pool . clone () ; let pool3 = pool . clone () ; let t1 = thread :: spawn (move | | { assert_eq ! (item1 . get_ref () , & String :: from ("hello")) ; pool2 . clear (key1) ; item1 }) ; let t2 = thread :: spawn (move | | { let item2 = pool3 . clone () . get_owned (key2) . unwrap () ; assert_eq ! (item2 . get_ref () , & String :: from ("world")) ; pool3 . clear (key1) ; item2 }) ; let item1 = t1 . join () . unwrap () ; let item2 = t2 . join () . unwrap () ; assert_eq ! (item1 . get_ref () , & String :: from ("hello")) ; assert_eq ! (item2 . get_ref () , & String :: from ("world")) ; }) ; }
    };
}

ownedref_ping_pong!()