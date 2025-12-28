macro_rules! deps {
    () => {
        Hasher!();
        State!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl BuildHasher for State { type Hasher = Hasher ; fn build_hasher (& self) -> Self :: Hasher { Hasher :: with_seed (self . 0) } }
    };
}

impl_55!();