macro_rules! deps {
    () => {
        UnordCollection!();
        UnordBag!();
    };
}

macro_rules! impl_666 {
    () => {
        deps!();
        impl < T > UnordCollection for UnordBag < T > { }
    };
}

impl_666!()