macro_rules! deps {
    () => {
        Min!();
        B0!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl Min < B0 > for B0 { type Output = B0 ; # [inline] fn min (self , _ : B0) -> B0 { self } }
    };
}

impl_26!()