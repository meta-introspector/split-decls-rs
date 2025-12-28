macro_rules! deps {
    () => {
        Channel!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        unsafe impl < T : Send > Sync for Channel < T > { }
    };
}

impl_63!()