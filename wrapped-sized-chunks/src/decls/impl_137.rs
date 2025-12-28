macro_rules! deps {
    () => {
        RawIndex!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < const N : usize > Sub < usize > for RawIndex < N > { type Output = RawIndex < N > ; # [inline] # [must_use] fn sub (self , other : usize) -> Self :: Output { let mut start = self . 0 ; while other > start { start += N ; } (start - other) . into () } }
    };
}

impl_137!()