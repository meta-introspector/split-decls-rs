macro_rules! deps {
    () => {
        Slab!();
    };
}

macro_rules! vacant_entry {
    () => {
        deps!();
        # [test] fn vacant_entry () { run_model ("vacant_entry" , | | { let slab = Arc :: new (Slab :: new ()) ; let entry = slab . vacant_entry () . unwrap () ; let key : usize = entry . key () ; let slab2 = slab . clone () ; let t1 = thread :: spawn (move | | { test_dbg ! (slab2 . get (key)) ; }) ; entry . insert ("hello world") ; t1 . join () . unwrap () ; assert_eq ! (slab . get (key) . expect ("get") , "hello world") ; }) ; }
    };
}

vacant_entry!()