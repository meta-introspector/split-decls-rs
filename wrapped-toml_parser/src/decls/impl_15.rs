macro_rules! deps {
    () => {
        ErrorSink!();
        ParseError!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl ErrorSink for () { fn report_error (& mut self , _error : ParseError) { } }
    };
}

impl_15!();