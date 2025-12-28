macro_rules! deps {
    () => {
        Once!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        unsafe impl < T : Send + Sync > Sync for Once < T > { }
    };
}

impl_21!()