macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
    };
}

macro_rules! test_scope_order {
    () => {
        deps!();
        macro_rules ! test_scope_order { ($ scope : ident => $ spawn : ident) => { { let builder = ThreadPoolBuilder :: new () . num_threads (1) ; let pool = builder . build () . unwrap () ; pool . install (|| { let vec = Mutex :: new (vec ! []) ; pool .$ scope (| scope | { let vec = & vec ; for i in 0 .. 10 { scope .$ spawn (move | _ | { vec . lock () . unwrap () . push (i) ; }) ; } }) ; vec . into_inner () . unwrap () }) } } ; }
    };
}

test_scope_order!();