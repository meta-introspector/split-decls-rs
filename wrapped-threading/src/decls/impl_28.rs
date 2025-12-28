macro_rules! deps {
    () => {
        Pool!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        unsafe impl Send for Pool { }
    };
}

impl_28!()