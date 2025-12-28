macro_rules! deps {
    () => {
        Service!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        unsafe impl Send for Service < '_ > { }
    };
}

impl_8!()