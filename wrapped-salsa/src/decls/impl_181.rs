macro_rules! deps {
    () => {
        Configuration!();
        Value!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        unsafe impl < C : Configuration > Sync for Value < C > { }
    };
}

impl_181!();