macro_rules! deps {
    () => {
        ATerm!();
    };
}

macro_rules! impl_543 {
    () => {
        deps!();
        impl < Rhs > Mul < Rhs > for ATerm { type Output = ATerm ; # [inline] fn mul (self , _ : Rhs) -> Self :: Output { ATerm } }
    };
}

impl_543!()