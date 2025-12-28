macro_rules! deps {
    () => {
        ConstValue!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl From < f64 > for ConstValue { # [inline] fn from (f : f64) -> Self { Number :: from_f64 (f) . map_or (ConstValue :: Null , ConstValue :: Number) } }
    };
}

impl_26!()