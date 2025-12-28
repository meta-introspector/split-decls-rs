macro_rules! deps {
    () => {
        CpuFeatures!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl CpuFeatures { pub const NONE : usize = 0 ; pub const AVX2 : usize = 1 ; }
    };
}

impl_88!();