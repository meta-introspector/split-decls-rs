macro_rules! deps {
    () => {
        Service!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        unsafe impl Send for Service < '_ > { }
    };
}

impl_39!();