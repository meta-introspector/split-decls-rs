macro_rules! deps {
    () => {
        Pool!();
    };
}

macro_rules! create_owned_mut_guard_2 {
    () => {
        deps!();
        # [test] fn create_owned_mut_guard_2 () { run_model ("create_owned_mut_guard_2" , | | { let pool = Arc :: new (Pool :: < String > :: new ()) ; let mut guard = pool . clone () . create_owned () . unwrap () ; let key : usize = guard . key () ; let pool2 = pool . clone () ; let pool3 = pool . clone () ; let t1 = thread :: spawn (move | | { test_dbg ! (pool2 . get (key)) ; }) ; guard . push_str ("Hello world") ; let t2 = thread :: spawn (move | | { test_dbg ! (pool3 . get (key)) ; }) ; drop (guard) ; t1 . join () . unwrap () ; t2 . join () . unwrap () ; }) ; }
    };
}

create_owned_mut_guard_2!();