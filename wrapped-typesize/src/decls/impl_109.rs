macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < T1 : TypeSize , T2 : TypeSize , T3 : TypeSize > TypeSize for (T1 , T2 , T3) { fn extra_size (& self) -> usize { self . 0 . extra_size () + self . 1 . extra_size () + self . 2 . extra_size () } }
    };
}

impl_109!();