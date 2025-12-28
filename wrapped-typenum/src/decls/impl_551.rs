macro_rules! deps {
    () => {
        ATerm!();
    };
}

macro_rules! impl_551 {
    () => {
        deps!();
        impl < Rhs > Div < Rhs > for ATerm { type Output = ATerm ; # [inline] fn div (self , _ : Rhs) -> Self :: Output { ATerm } }
    };
}

impl_551!()