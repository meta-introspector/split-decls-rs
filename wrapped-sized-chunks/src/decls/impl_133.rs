macro_rules! deps {
    () => {
        RawIndex!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl < const N : usize > Add for RawIndex < N > { type Output = RawIndex < N > ; # [inline] # [must_use] fn add (self , other : Self) -> Self :: Output { self + other . 0 } }
    };
}

impl_133!();