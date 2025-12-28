macro_rules! impl_23 {
    () => {
        impl < T , F > crate :: sealed :: Sealed < T > for F where F : Fn (& T) -> tracing :: Span { }
    };
}

impl_23!()