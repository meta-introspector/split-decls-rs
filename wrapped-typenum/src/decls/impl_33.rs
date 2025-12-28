macro_rules! deps {
    () => {
        Max!();
        B1!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl Max < B1 > for B1 { type Output = B1 ; # [inline] fn max (self , _ : B1) -> B1 { self } }
    };
}

impl_33!();