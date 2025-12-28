macro_rules! deps {
    () => {
        OwnedSlice!();
    };
}

macro_rules! impl_347 {
    () => {
        deps!();
        unsafe impl Sync for OwnedSlice { }
    };
}

impl_347!()