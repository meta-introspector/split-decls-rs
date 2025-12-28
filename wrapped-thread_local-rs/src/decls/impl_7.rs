macro_rules! deps {
    () => {
        ThreadLocal!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        unsafe impl < T : Send > Sync for ThreadLocal < T > { }
    };
}

impl_7!()