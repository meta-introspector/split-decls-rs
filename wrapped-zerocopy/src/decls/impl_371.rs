macro_rules! deps {
    () => {
        InvariantsEq!();
    };
}

macro_rules! impl_371 {
    () => {
        deps!();
        unsafe impl < T : ? Sized > InvariantsEq < ManuallyDrop < T > > for T { }
    };
}

impl_371!();