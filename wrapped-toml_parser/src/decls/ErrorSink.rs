macro_rules! deps {
    () => {
        ParseError!();
    };
}

macro_rules! ErrorSink {
    () => {
        deps!();
        pub trait ErrorSink { fn report_error (& mut self , error : ParseError) ; }
    };
}

ErrorSink!();