macro_rules! deps {
    () => {
        InstrumentableService!();
        Service!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < S , R > InstrumentableService < R > for S where S : Service < R > + Sized { }
    };
}

impl_21!()