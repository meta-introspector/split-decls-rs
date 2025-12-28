macro_rules! deps {
    () => {
        RawIndex!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < const N : usize > PartialEq for RawIndex < N > { # [inline] # [must_use] fn eq (& self , other : & Self) -> bool { self . 0 == other . 0 } }
    };
}

impl_131!();