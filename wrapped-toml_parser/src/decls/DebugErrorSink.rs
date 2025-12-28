macro_rules! deps {
    () => {
        ErrorSink!();
    };
}

macro_rules! DebugErrorSink {
    () => {
        deps!();
        pub (crate) struct DebugErrorSink < 's > { sink : & 's mut dyn ErrorSink , }
    };
}

DebugErrorSink!()