macro_rules! deps {
    () => {
        Slot!();
        Config!();
    };
}

macro_rules! FreeList {
    () => {
        deps!();
        pub (crate) trait FreeList < C > { fn push < T > (& self , new_head : usize , slot : & Slot < T , C >) where C : cfg :: Config ; }
    };
}

FreeList!()