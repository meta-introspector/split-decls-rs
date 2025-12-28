macro_rules! deps {
    () => {
        RawIndex!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl < const N : usize > Add < usize > for RawIndex < N > { type Output = RawIndex < N > ; # [inline] # [must_use] fn add (self , other : usize) -> Self :: Output { let mut result = self . 0 + other ; while result >= N { result -= N ; } result . into () } }
    };
}

impl_134!();