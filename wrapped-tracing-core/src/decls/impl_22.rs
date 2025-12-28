macro_rules! deps {
    () => {
        Once!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for Once < T > { }
    };
}

impl_22!();