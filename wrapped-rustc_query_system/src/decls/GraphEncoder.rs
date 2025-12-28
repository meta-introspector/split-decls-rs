macro_rules! deps {
    () => {
        Deps!();
        EncoderState!();
        DepGraphQuery!();
    };
}

macro_rules! GraphEncoder {
    () => {
        deps!();
        pub (crate) struct GraphEncoder < D : Deps > { profiler : SelfProfilerRef , status : EncoderState < D > , record_graph : Option < Lock < DepGraphQuery > > , }
    };
}

GraphEncoder!()