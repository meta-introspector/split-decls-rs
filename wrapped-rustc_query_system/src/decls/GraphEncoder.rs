macro_rules! deps {
    () => {
        Deps!();
        DepGraphQuery!();
        EncoderState!();
    };
}

macro_rules! GraphEncoder {
    () => {
        deps!();
        pub (crate) struct GraphEncoder < D : Deps > { profiler : SelfProfilerRef , status : EncoderState < D > , record_graph : Option < Lock < DepGraphQuery > > , }
    };
}

GraphEncoder!();