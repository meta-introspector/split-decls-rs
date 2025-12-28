macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        unsafe impl < T : Send > Send for ThinVec < T > { }
    };
}

impl_18!()