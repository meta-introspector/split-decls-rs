macro_rules! deps {
    () => {
        Channel!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for Channel < T > { }
    };
}

impl_62!();