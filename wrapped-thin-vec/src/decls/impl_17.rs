macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        unsafe impl < T : Sync > Sync for ThinVec < T > { }
    };
}

impl_17!();