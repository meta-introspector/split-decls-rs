macro_rules! deps {
    () => {
        Slab!();
    };
}

macro_rules! owned_entrys_outlive_slab {
    () => {
        deps!();
        # [test] fn owned_entrys_outlive_slab () { run_model ("owned_entrys_outlive_slab" , | | { let slab = Arc :: new (Slab :: < alloc :: Track < String > > :: new ()) ; let key1 = slab . insert (alloc :: Track :: new (String :: from ("hello"))) . expect ("insert item 1") ; let key2 = slab . insert (alloc :: Track :: new (String :: from ("goodbye"))) . expect ("insert item 2") ; let item1_1 = slab . clone () . get_owned (key1) . expect ("get key1") ; let item1_2 = slab . clone () . get_owned (key1) . expect ("get key1 again") ; let item2 = slab . clone () . get_owned (key2) . expect ("get key2") ; drop (slab) ; let t1 = thread :: spawn (move | | { assert_eq ! (item1_1 . get_ref () , & String :: from ("hello")) ; drop (item1_1) ; }) ; let t2 = thread :: spawn (move | | { assert_eq ! (item2 . get_ref () , & String :: from ("goodbye")) ; drop (item2) ; }) ; t1 . join () . unwrap () ; t2 . join () . unwrap () ; assert_eq ! (item1_2 . get_ref () , & String :: from ("hello")) ; }) ; }
    };
}

owned_entrys_outlive_slab!()