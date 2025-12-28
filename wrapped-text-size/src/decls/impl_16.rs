macro_rules! deps {
    () => {
        TextRange!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < A > AddAssign < A > for TextRange where TextRange : Add < A , Output = TextRange > , { # [inline] fn add_assign (& mut self , rhs : A) { * self = * self + rhs } }
    };
}

impl_16!()