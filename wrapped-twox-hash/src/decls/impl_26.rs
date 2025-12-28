macro_rules! deps {
    () => {
        State!();
        Hasher!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl BuildHasher for State { type Hasher = Hasher ; fn build_hasher (& self) -> Self :: Hasher { Hasher :: with_seed (self . 0) } }
    };
}

impl_26!()