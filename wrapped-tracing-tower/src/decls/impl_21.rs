macro_rules! deps {
    () => {
        Service!();
        InstrumentableService!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < S , R > InstrumentableService < R > for S where S : Service < R > + Sized { }
    };
}

impl_21!();