macro_rules! deps {
    () => {
        Z0!();
        ATerm!();
    };
}

macro_rules! impl_545 {
    () => {
        deps!();
        impl Mul < ATerm > for Z0 { type Output = ATerm ; # [inline] fn mul (self , _ : ATerm) -> Self :: Output { ATerm } }
    };
}

impl_545!();