macro_rules! deps {
    () => {
        TextRange!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < S > SubAssign < S > for TextRange where TextRange : Sub < S , Output = TextRange > , { # [inline] fn sub_assign (& mut self , rhs : S) { * self = * self - rhs } }
    };
}

impl_17!()