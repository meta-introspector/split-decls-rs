macro_rules! deps {
    () => {
        ExtendedCommand!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        unsafe impl Send for ExtendedCommand { }
    };
}

impl_34!();