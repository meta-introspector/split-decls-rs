macro_rules! deps {
    () => {
        ScopedKey!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        unsafe impl < T > Sync for ScopedKey < T > { }
    };
}

impl_2!()