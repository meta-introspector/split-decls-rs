macro_rules! deps {
    () => {
        Halves!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl Halves for u128 { type Output = u64 ; # [inline] fn upper_half (self) -> Self :: Output { (self >> 64) as _ } # [inline] fn lower_half (self) -> Self :: Output { self as _ } }
    };
}

impl_88!()