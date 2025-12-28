macro_rules! deps {
    () => {
        Unaligned!();
        Alignment!();
    };
}

macro_rules! impl_327 {
    () => {
        deps!();
        impl Alignment for Unaligned { }
    };
}

impl_327!();