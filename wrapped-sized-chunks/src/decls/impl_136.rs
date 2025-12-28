macro_rules! deps {
    () => {
        RawIndex!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl < const N : usize > Sub for RawIndex < N > { type Output = RawIndex < N > ; # [inline] # [must_use] fn sub (self , other : Self) -> Self :: Output { self - other . 0 } }
    };
}

impl_136!();