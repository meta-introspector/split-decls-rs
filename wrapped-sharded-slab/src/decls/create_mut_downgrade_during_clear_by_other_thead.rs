macro_rules! deps {
    () => {
        Pool!();
    };
}

macro_rules! create_mut_downgrade_during_clear_by_other_thead {
    () => {
        deps!();
        # [test] fn create_mut_downgrade_during_clear_by_other_thead () { run_model ("create_mut_downgrade_during_clear_by_other_thread" , | | { let pool = Arc :: new (Pool :: < String > :: new ()) ; let mut guard = pool . clone () . create_owned () . unwrap () ; let key : usize = guard . key () ; guard . push_str ("Hello world") ; let pool2 = pool . clone () ; let t1 = thread :: spawn (move | | { let guard = guard . downgrade () ; assert_eq ! (guard , "Hello world" . to_owned ()) ; drop (guard) ; }) ; let t2 = thread :: spawn (move | | { test_dbg ! (pool2 . clear (key)) ; }) ; test_dbg ! (pool . get (key)) ; t1 . join () . unwrap () ; t2 . join () . unwrap () ; }) ; }
    };
}

create_mut_downgrade_during_clear_by_other_thead!()