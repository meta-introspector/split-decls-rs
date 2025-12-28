macro_rules! deps {
    () => {
        BecauseExclusive!();
        Exclusive!();
        Read!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        impl < T : ? Sized > Read < Exclusive , BecauseExclusive > for T { }
    };
}

impl_343!();