macro_rules! deps {
    () => {
        UnordBag!();
    };
}

macro_rules! impl_672 {
    () => {
        deps!();
        impl < T > ! IntoIterator for UnordBag < T > { }
    };
}

impl_672!();