macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < 's , T > Eq for SliceVec < 's , T > where T : Eq { }
    };
}

impl_101!()