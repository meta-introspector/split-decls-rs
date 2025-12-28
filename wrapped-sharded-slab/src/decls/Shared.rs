macro_rules! deps {
    () => {
        TransferStack!();
        Slots!();
    };
}

macro_rules! Shared {
    () => {
        deps!();
        pub (crate) struct Shared < T , C > { # [doc = " The remote free list"] # [doc = ""] # [doc = " Slots freed from a remote thread are pushed onto this list."] remote : stack :: TransferStack < C > , size : usize , prev_sz : usize , slab : UnsafeCell < Option < Slots < T , C > > > , }
    };
}

Shared!();