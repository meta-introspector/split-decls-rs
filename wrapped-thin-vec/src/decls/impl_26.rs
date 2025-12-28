macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < T > Borrow < [T] > for ThinVec < T > { fn borrow (& self) -> & [T] { self . as_slice () } }
    };
}

impl_26!()