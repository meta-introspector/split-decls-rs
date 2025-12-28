macro_rules! deps {
    () => {
        JobRef!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        unsafe impl Send for JobRef { }
    };
}

impl_34!()