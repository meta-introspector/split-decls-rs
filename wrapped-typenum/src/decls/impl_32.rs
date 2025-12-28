macro_rules! deps {
    () => {
        Max!();
        B0!();
        B1!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl Max < B0 > for B1 { type Output = B1 ; # [inline] fn max (self , _ : B0) -> B1 { self } }
    };
}

impl_32!()