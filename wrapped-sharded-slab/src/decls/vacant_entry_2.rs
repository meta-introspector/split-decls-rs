macro_rules! deps {
    () => {
        Slab!();
    };
}

macro_rules! vacant_entry_2 {
    () => {
        deps!();
        # [test] fn vacant_entry_2 () { run_model ("vacant_entry_2" , | | { let slab = Arc :: new (Slab :: new ()) ; let entry = slab . vacant_entry () . unwrap () ; let key : usize = entry . key () ; let slab2 = slab . clone () ; let slab3 = slab . clone () ; let t1 = thread :: spawn (move | | { test_dbg ! (slab2 . get (key)) ; }) ; entry . insert ("hello world") ; let t2 = thread :: spawn (move | | { test_dbg ! (slab3 . get (key)) ; }) ; t1 . join () . unwrap () ; t2 . join () . unwrap () ; assert_eq ! (slab . get (key) . expect ("get") , "hello world") ; }) ; }
    };
}

vacant_entry_2!()