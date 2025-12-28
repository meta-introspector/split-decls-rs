macro_rules! deps {
    () => {
        Timespec!();
    };
}

macro_rules! impl_1687 {
    () => {
        deps!();
        impl Neg for Timespec { type Output = Self ; fn neg (self) -> Self { Self :: default () - self } }
    };
}

impl_1687!();