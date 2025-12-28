macro_rules! deps {
    () => {
        TextSize!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < S > SubAssign < S > for TextSize where TextSize : Sub < S , Output = TextSize > , { # [inline] fn sub_assign (& mut self , rhs : S) { * self = * self - rhs } }
    };
}

impl_31!()