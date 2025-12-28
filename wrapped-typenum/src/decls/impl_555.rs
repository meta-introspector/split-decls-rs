macro_rules! deps {
    () => {
        ATerm!();
    };
}

macro_rules! impl_555 {
    () => {
        deps!();
        impl < Rhs > Rem < Rhs > for ATerm { type Output = ATerm ; # [inline] fn rem (self , _ : Rhs) -> Self :: Output { ATerm } }
    };
}

impl_555!()