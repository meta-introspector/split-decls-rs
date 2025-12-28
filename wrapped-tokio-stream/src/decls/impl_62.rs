macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        unsafe impl < T > Send for Pending < T > { }
    };
}

impl_62!()