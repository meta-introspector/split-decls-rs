macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl From < f32 > for ConstValue { # [inline] fn from (f : f32) -> Self { From :: from (f as f64) } }
    };
}

impl_102!()