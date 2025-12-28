macro_rules! deps {
    () => {
        ThreadLocal!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        unsafe impl < T : Send > Sync for ThreadLocal < T > { }
    };
}

impl_28!()