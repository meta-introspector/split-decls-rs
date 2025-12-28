macro_rules! deps {
    () => {
        Page!();
    };
}

macro_rules! impl_324 {
    () => {
        deps!();
        unsafe impl Send for Page { }
    };
}

impl_324!()