macro_rules! deps {
    () => {
        RawIndex!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl < const N : usize > AddAssign < usize > for RawIndex < N > { # [inline] fn add_assign (& mut self , other : usize) { self . 0 += other ; while self . 0 >= N { self . 0 -= N ; } } }
    };
}

impl_135!();