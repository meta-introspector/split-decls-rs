macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < T1 : TypeSize , T2 : TypeSize > TypeSize for (T1 , T2) { fn extra_size (& self) -> usize { self . 0 . extra_size () + self . 1 . extra_size () } }
    };
}

impl_108!()