macro_rules! deps {
    () => {
        Pool!();
    };
}

macro_rules! create_mut_guard_prevents_access {
    () => {
        deps!();
        # [test] fn create_mut_guard_prevents_access () { run_model ("create_mut_guard_prevents_access" , | | { let pool = Arc :: new (Pool :: < String > :: new ()) ; let guard = pool . create () . unwrap () ; let key : usize = guard . key () ; let pool2 = pool . clone () ; thread :: spawn (move | | { assert ! (pool2 . get (key) . is_none ()) ; }) . join () . unwrap () ; }) ; }
    };
}

create_mut_guard_prevents_access!()