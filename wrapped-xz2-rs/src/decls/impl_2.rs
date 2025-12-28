macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        unsafe impl Sync for Stream { }
    };
}

impl_2!()