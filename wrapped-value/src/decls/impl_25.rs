macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl From < f32 > for ConstValue { # [inline] fn from (f : f32) -> Self { From :: from (f as f64) } }
    };
}

impl_25!()