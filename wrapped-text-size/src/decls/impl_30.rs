macro_rules! deps {
    () => {
        TextSize!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < A > AddAssign < A > for TextSize where TextSize : Add < A , Output = TextSize > , { # [inline] fn add_assign (& mut self , rhs : A) { * self = * self + rhs } }
    };
}

impl_30!()