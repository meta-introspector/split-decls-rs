macro_rules! deps {
    () => {
        ActualSpan!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl ActualSpan { pub (crate) fn new (id : tracing_core :: span :: Id , metadata : Option < & 'static tracing_core :: Metadata < 'static > > ,) -> Self { Self { id , metadata } } # [doc = " The Id of the actual span."] pub (crate) fn id (& self) -> tracing_core :: span :: Id { self . id . clone () } # [doc = " The metadata for the actual span if it is available."] pub (crate) fn metadata (& self) -> Option < & 'static tracing_core :: Metadata < 'static > > { self . metadata } }
    };
}

impl_50!();