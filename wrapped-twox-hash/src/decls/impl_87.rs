macro_rules! deps {
    () => {
        Halves!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl Halves for u64 { type Output = u32 ; # [inline] fn upper_half (self) -> Self :: Output { (self >> 32) as _ } # [inline] fn lower_half (self) -> Self :: Output { self as _ } }
    };
}

impl_87!()