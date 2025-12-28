macro_rules! deps {
    () => {
        B0!();
        Max!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl Max < B0 > for B0 { type Output = B0 ; # [inline] fn max (self , _ : B0) -> B0 { self } }
    };
}

impl_30!()