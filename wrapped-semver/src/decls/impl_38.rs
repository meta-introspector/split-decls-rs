macro_rules! deps {
    () => {
        Identifier!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        unsafe impl Send for Identifier { }
    };
}

impl_38!();