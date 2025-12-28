macro_rules! deps {
    () => {
        InvariantsEq!();
    };
}

macro_rules! impl_364 {
    () => {
        deps!();
        unsafe impl < T : ? Sized > InvariantsEq < T > for T { }
    };
}

impl_364!()