macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl TypeSize for Box < serde_json :: value :: RawValue > { fn extra_size (& self) -> usize { core :: mem :: size_of :: < u8 > () * self . get () . len () } }
    };
}

impl_49!()