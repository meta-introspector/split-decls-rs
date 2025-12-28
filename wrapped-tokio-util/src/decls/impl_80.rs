macro_rules! deps {
    () => {
        ReusableBoxFuture!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        unsafe impl < T > Sync for ReusableBoxFuture < '_ , T > { }
    };
}

impl_80!();