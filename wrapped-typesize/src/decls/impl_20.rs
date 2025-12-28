macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < T : TypeSize > TypeSize for SharedValue < T > { fn extra_size (& self) -> usize { self . get () . extra_size () } }
    };
}

impl_20!();