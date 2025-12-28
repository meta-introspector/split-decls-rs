macro_rules! deps {
    () => {
        PartialDiv!();
        ATerm!();
    };
}

macro_rules! impl_553 {
    () => {
        deps!();
        impl < Rhs > PartialDiv < Rhs > for ATerm { type Output = ATerm ; # [inline] fn partial_div (self , _ : Rhs) -> Self :: Output { ATerm } }
    };
}

impl_553!();