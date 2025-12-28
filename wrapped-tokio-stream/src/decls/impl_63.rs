macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        unsafe impl < T > Sync for Pending < T > { }
    };
}

impl_63!();