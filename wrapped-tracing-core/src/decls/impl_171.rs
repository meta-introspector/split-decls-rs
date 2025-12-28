macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl Eq for Field { }
    };
}

impl_171!();