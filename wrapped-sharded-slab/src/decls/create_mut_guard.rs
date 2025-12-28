macro_rules! deps {
    () => {
        Pool!();
    };
}

macro_rules! create_mut_guard {
    () => {
        deps!();
        # [test] fn create_mut_guard () { run_model ("create_mut_guard" , | | { let pool = Arc :: new (Pool :: < String > :: new ()) ; let mut guard = pool . create () . unwrap () ; let key : usize = guard . key () ; let pool2 = pool . clone () ; let t1 = thread :: spawn (move | | { test_dbg ! (pool2 . get (key)) ; }) ; guard . push_str ("Hello world") ; drop (guard) ; t1 . join () . unwrap () ; }) ; }
    };
}

create_mut_guard!()