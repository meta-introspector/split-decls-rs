macro_rules! deps {
    () => {
        ExtendedCommand!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        unsafe impl Sync for ExtendedCommand { }
    };
}

impl_35!();