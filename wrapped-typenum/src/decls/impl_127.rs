macro_rules! deps {
    () => {
        Min!();
        Z0!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl Min < Z0 > for Z0 { type Output = Z0 ; # [inline] fn min (self , _ : Z0) -> Self :: Output { self } }
    };
}

impl_127!();