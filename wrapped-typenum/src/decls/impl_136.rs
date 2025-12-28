macro_rules! deps {
    () => {
        Max!();
        Z0!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl Max < Z0 > for Z0 { type Output = Z0 ; # [inline] fn max (self , _ : Z0) -> Self :: Output { self } }
    };
}

impl_136!()