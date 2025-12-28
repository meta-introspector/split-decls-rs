macro_rules! deps {
    () => {
        Identifier!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        unsafe impl Sync for Identifier { }
    };
}

impl_39!()