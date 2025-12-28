macro_rules! deps {
    () => {
        Pool!();
    };
}

macro_rules! create_mut_downgrade_during_clear {
    () => {
        deps!();
        # [test] fn create_mut_downgrade_during_clear () { run_model ("create_mut_downgrade_during_clear" , | | { let pool = Arc :: new (Pool :: < String > :: new ()) ; let mut guard = pool . create () . unwrap () ; let key : usize = guard . key () ; guard . push_str ("Hello world") ; let pool2 = pool . clone () ; let guard = guard . downgrade () ; let t1 = thread :: spawn (move | | { test_dbg ! (pool2 . clear (key)) ; }) ; t1 . join () . unwrap () ; assert_eq ! (guard , "Hello world" . to_owned ()) ; drop (guard) ; assert ! (pool . get (key) . is_none ()) ; }) ; }
    };
}

create_mut_downgrade_during_clear!();