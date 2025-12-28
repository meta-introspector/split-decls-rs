macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl Eq for Key { }
    };
}

impl_134!();