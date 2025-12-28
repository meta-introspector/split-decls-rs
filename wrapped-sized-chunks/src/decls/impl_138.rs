macro_rules! deps {
    () => {
        RawIndex!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl < const N : usize > SubAssign < usize > for RawIndex < N > { # [inline] fn sub_assign (& mut self , other : usize) { while other > self . 0 { self . 0 += N ; } self . 0 -= other ; } }
    };
}

impl_138!();