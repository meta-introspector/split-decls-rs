macro_rules! ActualSpan {
    () => {
        pub (crate) struct ActualSpan { id : tracing_core :: span :: Id , metadata : Option < & 'static tracing_core :: Metadata < 'static > > , }
    };
}

ActualSpan!();