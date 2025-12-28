macro_rules! deps {
    () => {
        ExtendedCommand!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        unsafe impl Send for ExtendedCommand { }
    };
}

impl_3!()