macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        unsafe impl Send for Stream { }
    };
}

impl_1!()