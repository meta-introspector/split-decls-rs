macro_rules! deps {
    () => {
        Immutable!();
        Read!();
        BecauseImmutable!();
        Aliasing!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl < A : Aliasing , T : ? Sized + crate :: Immutable > Read < A , BecauseImmutable > for T { }
    };
}

impl_342!();