macro_rules! deps {
    () => {
        FloatLength!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl FloatLength { pub fn bits (self) -> usize { match self { FloatLength :: F16 => 16 , FloatLength :: F32 => 32 , FloatLength :: F64 => 64 , FloatLength :: F128 => 128 , } } }
    };
}

impl_36!();