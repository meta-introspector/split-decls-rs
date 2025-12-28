macro_rules! deps {
    () => {
        Pool!();
    };
}

macro_rules! create_owned_mut_guard_downgrade {
    () => {
        deps!();
        # [test] fn create_owned_mut_guard_downgrade () { run_model ("create_owned_mut_guard_downgrade" , | | { let pool = Arc :: new (Pool :: < String > :: new ()) ; let mut guard = pool . clone () . create_owned () . unwrap () ; guard . push_str ("Hello world") ; let key : usize = guard . key () ; let pool2 = pool . clone () ; let pool3 = pool . clone () ; let t1 = thread :: spawn (move | | { test_dbg ! (pool2 . get (key)) ; }) ; let guard = guard . downgrade () ; let t2 = thread :: spawn (move | | { assert_eq ! (pool3 . get (key) . unwrap () , "Hello world" . to_owned ()) ; }) ; t1 . join () . unwrap () ; t2 . join () . unwrap () ; assert_eq ! (guard , "Hello world" . to_owned ()) ; }) ; }
    };
}

create_owned_mut_guard_downgrade!();