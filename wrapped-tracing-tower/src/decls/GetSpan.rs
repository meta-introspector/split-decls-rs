macro_rules! GetSpan {
    () => {
        pub trait GetSpan < T > : crate :: sealed :: Sealed < T > { fn span_for (& self , target : & T) -> tracing :: Span ; }
    };
}

GetSpan!();