macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        unsafe impl < T > Send for Empty < T > { }
    };
}

impl_45!()