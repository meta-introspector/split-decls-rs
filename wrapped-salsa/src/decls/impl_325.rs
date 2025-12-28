macro_rules! deps {
    () => {
        Page!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        unsafe impl Sync for Page { }
    };
}

impl_325!()