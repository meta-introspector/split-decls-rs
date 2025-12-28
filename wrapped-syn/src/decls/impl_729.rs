macro_rules! deps {
    () => {
        ThreadBound!();
    };
}

macro_rules! impl_729 {
    () => {
        deps!();
        unsafe impl < T > Sync for ThreadBound < T > { }
    };
}

impl_729!()