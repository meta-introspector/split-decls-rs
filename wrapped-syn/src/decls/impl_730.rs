macro_rules! deps {
    () => {
        ThreadBound!();
    };
}

macro_rules! impl_730 {
    () => {
        deps!();
        unsafe impl < T : Copy > Send for ThreadBound < T > { }
    };
}

impl_730!();