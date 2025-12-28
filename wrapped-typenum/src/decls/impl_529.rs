macro_rules! deps {
    () => {
        TypeArray!();
        ATerm!();
    };
}

macro_rules! impl_529 {
    () => {
        deps!();
        impl TypeArray for ATerm { }
    };
}

impl_529!();