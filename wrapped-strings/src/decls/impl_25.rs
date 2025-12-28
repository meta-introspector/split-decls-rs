macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        unsafe impl Send for HSTRING { }
    };
}

impl_25!()