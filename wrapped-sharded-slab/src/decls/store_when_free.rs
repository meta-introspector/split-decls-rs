macro_rules! deps {
    () => {
        Config!();
        Slab!();
    };
}

macro_rules! store_when_free {
    () => {
        deps!();
        fn store_when_free < C : crate :: Config > (slab : & Arc < Slab < usize , C > > , t : usize) -> usize { loop { test_println ! ("try store {:?}" , t) ; if let Some (key) = slab . insert (t) { test_println ! ("inserted at {:#x}" , key) ; return key ; } test_println ! ("retrying; slab is full...") ; thread :: yield_now () ; } }
    };
}

store_when_free!()