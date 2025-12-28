macro_rules! deps {
    () => {
        B0!();
        Min!();
        B1!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl Min < B1 > for B0 { type Output = B0 ; # [inline] fn min (self , _ : B1) -> B0 { self } }
    };
}

impl_27!()