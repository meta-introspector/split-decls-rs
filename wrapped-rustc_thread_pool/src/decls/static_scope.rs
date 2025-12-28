macro_rules! deps {
    () => {
        Scope!();
    };
}

macro_rules! static_scope {
    () => {
        deps!();
        # [test] fn static_scope () { static COUNTER : AtomicUsize = AtomicUsize :: new (0) ; let mut range = 0 .. 100 ; let sum = range . clone () . sum () ; let iter = & mut range ; COUNTER . store (0 , Ordering :: Relaxed) ; scope (| s : & Scope < 'static > | { for i in iter { s . spawn (move | _ | { COUNTER . fetch_add (i , Ordering :: Relaxed) ; }) ; } }) ; assert_eq ! (COUNTER . load (Ordering :: Relaxed) , sum) ; }
    };
}

static_scope!();