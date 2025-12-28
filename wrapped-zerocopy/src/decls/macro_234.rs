macro_rules! deps {
    () => {
        Unalign!();
    };
}

macro_rules! macro_234 {
    () => {
        deps!();
        impl_size_eq ! (char , Unalign < u32 >) ;
    };
}

macro_234!();