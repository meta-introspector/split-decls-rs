macro_rules! deps {
    () => {
        AsTrace!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl AsTrace for log :: Level { type Trace = tracing_core :: Level ; # [inline] fn as_trace (& self) -> tracing_core :: Level { match self { log :: Level :: Error => tracing_core :: Level :: ERROR , log :: Level :: Warn => tracing_core :: Level :: WARN , log :: Level :: Info => tracing_core :: Level :: INFO , log :: Level :: Debug => tracing_core :: Level :: DEBUG , log :: Level :: Trace => tracing_core :: Level :: TRACE , } } }
    };
}

impl_57!()