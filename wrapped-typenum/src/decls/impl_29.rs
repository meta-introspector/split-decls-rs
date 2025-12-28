macro_rules! deps {
    () => {
        B1!();
        Min!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Min < B1 > for B1 { type Output = B1 ; # [inline] fn min (self , _ : B1) -> B1 { self } }
    };
}

impl_29!()