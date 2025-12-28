macro_rules! deps {
    () => {
        Pool!();
        Generation!();
    };
}

macro_rules! clear_local_and_reuse {
    () => {
        deps!();
        # [test] fn clear_local_and_reuse () { run_model ("take_remote_and_reuse" , | | { let pool = Arc :: new (Pool :: new_with_config :: < TinyConfig > ()) ; let idx1 = pool . create_with (| item : & mut String | { item . push_str ("hello world") ; }) . expect ("create_with") ; let idx2 = pool . create_with (| item | item . push_str ("foo")) . expect ("create_with") ; let idx3 = pool . create_with (| item | item . push_str ("bar")) . expect ("create_with") ; assert_eq ! (pool . get (idx1) . unwrap () , String :: from ("hello world")) ; assert_eq ! (pool . get (idx2) . unwrap () , String :: from ("foo")) ; assert_eq ! (pool . get (idx3) . unwrap () , String :: from ("bar")) ; let first = idx1 & (! crate :: page :: slot :: Generation :: < TinyConfig > :: MASK) ; assert ! (pool . clear (idx1)) ; let idx1 = pool . create_with (move | item | item . push_str ("h")) . expect ("create_with") ; let second = idx1 & (! crate :: page :: slot :: Generation :: < TinyConfig > :: MASK) ; assert_eq ! (first , second) ; assert ! (pool . get (idx1) . unwrap () . capacity () >= 11) ; }) }
    };
}

clear_local_and_reuse!()