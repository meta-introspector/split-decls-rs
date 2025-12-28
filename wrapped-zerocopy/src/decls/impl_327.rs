macro_rules! deps {
    () => {
        Alignment!();
        Unaligned!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl Alignment for Unaligned { }
    };
}

impl_327!()