macro_rules! deps {
    () => {
        ParseError!();
        ErrorSink!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl ErrorSink for Option < ParseError > { fn report_error (& mut self , error : ParseError) { self . get_or_insert (error) ; } }
    };
}

impl_16!();