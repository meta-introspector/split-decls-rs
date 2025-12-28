macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        unsafe impl Sync for HSTRING { }
    };
}

impl_26!();