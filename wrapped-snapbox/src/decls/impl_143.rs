macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl Eq for Data { }
    };
}

impl_143!();