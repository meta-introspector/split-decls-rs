macro_rules! deps {
    () => {
        InstrumentableService!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl < S , R > InstrumentableService < R > for S where S : Service < R > + Sized { }
    };
}

impl_5!()