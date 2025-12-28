macro_rules! deps {
    () => {
        Pointers!();
    };
}

macro_rules! impl_302 {
    () => {
        deps!();
        unsafe impl < T : Sync > Sync for Pointers < T > { }
    };
}

impl_302!();