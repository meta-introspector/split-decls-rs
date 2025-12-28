macro_rules! deps {
    () => {
        OwnedSlice!();
    };
}

macro_rules! impl_346 {
    () => {
        deps!();
        unsafe impl Send for OwnedSlice { }
    };
}

impl_346!();