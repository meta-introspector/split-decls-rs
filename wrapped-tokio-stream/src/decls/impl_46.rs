macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        unsafe impl < T > Sync for Empty < T > { }
    };
}

impl_46!()