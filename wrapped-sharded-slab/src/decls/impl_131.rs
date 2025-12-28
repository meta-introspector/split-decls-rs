macro_rules! deps {
    () => {
        Config!();
        Local!();
        FreeList!();
        Slot!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < C : cfg :: Config > FreeList < C > for Local { fn push < T > (& self , new_head : usize , slot : & Slot < T , C >) { slot . set_next (self . head ()) ; self . set_head (new_head) ; } }
    };
}

impl_131!();