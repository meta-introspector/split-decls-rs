macro_rules! deps {
    () => {
        Pool!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        unsafe impl Sync for Pool { }
    };
}

impl_27!();