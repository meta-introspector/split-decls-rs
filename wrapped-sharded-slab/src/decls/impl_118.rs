macro_rules! deps {
    () => {
        Slot!();
        Config!();
        TransferStack!();
        FreeList!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < C : cfg :: Config > super :: FreeList < C > for TransferStack < C > { fn push < T > (& self , new_head : usize , slot : & super :: Slot < T , C >) { self . push (new_head , | next | slot . set_next (next)) } }
    };
}

impl_118!()