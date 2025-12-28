macro_rules! deps {
    () => {
        Service!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        unsafe impl Sync for Service < '_ > { }
    };
}

impl_9!()