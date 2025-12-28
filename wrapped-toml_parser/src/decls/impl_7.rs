macro_rules! deps {
    () => {
        DebugErrorSink!();
        ErrorSink!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < 's > DebugErrorSink < 's > { pub (crate) fn new (sink : & 's mut dyn ErrorSink) -> Self { Self { sink } } }
    };
}

impl_7!()