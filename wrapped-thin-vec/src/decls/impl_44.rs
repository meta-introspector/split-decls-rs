macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < T > Eq for ThinVec < T > where T : Eq { }
    };
}

impl_44!()