macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < T : TypeSize > TypeSize for RwLock < T > { fn extra_size (& self) -> usize { self . read () . extra_size () } }
    };
}

impl_19!();