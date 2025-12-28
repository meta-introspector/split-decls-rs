macro_rules! deps {
    () => {
        InvariantsEq!();
    };
}

macro_rules! impl_370 {
    () => {
        deps!();
        unsafe impl < T : ? Sized > InvariantsEq < T > for ManuallyDrop < T > { }
    };
}

impl_370!();