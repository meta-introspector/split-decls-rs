macro_rules! InstrumentedService {
    () => {
        pub type InstrumentedService < S , R > = service_span :: Service < request_span :: Service < S , R > > ;
    };
}

InstrumentedService!()