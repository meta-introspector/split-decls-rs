macro_rules! deps {
    () => {
        B1!();
        Max!();
        B0!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl Max < B1 > for B0 { type Output = B1 ; # [inline] fn max (self , rhs : B1) -> B1 { rhs } }
    };
}

impl_31!();