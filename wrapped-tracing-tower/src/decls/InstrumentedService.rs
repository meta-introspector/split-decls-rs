macro_rules! deps {
    () => {
        Service!();
    };
}

macro_rules! InstrumentedService {
    () => {
        deps!();
        pub type InstrumentedService < S , R > = service_span :: Service < request_span :: Service < S , R > > ;
    };
}

InstrumentedService!();