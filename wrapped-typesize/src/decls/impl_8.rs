macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < H > TypeSize for core :: hash :: BuildHasherDefault < H > { }
    };
}

impl_8!();