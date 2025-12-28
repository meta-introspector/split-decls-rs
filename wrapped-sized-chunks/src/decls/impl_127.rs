macro_rules! deps {
    () => {
        RawIndex!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < const N : usize > Clone for RawIndex < N > { # [inline] # [must_use] fn clone (& self) -> Self { self . 0 . into () } }
    };
}

impl_127!();