macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! test_spawn_order {
    () => {
        deps!();
        macro_rules ! test_spawn_order { ($ spawn : ident) => { { let builder = ThreadPoolBuilder :: new () . num_threads (1) ; let pool = & builder . build () . unwrap () ; let (tx , rx) = channel () ; pool . install (move || { for i in 0 .. 10 { let tx = tx . clone () ; pool .$ spawn (move || { tx . send (i) . unwrap () ; }) ; } }) ; rx . iter () . collect ::< Vec < i32 >> () } } ; }
    };
}

test_spawn_order!();