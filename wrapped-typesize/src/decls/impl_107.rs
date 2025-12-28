macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < T1 : TypeSize > TypeSize for (T1 ,) { fn extra_size (& self) -> usize { self . 0 . extra_size () } }
    };
}

impl_107!();