macro_rules! deps {
    () => {
        ParseError!();
        ErrorSink!();
        DebugErrorSink!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl ErrorSink for DebugErrorSink < '_ > { fn report_error (& mut self , error : crate :: ParseError) { render_event (error . unexpected () , & format ! ("{error:?}") , anstyle :: AnsiColor :: Red . on_default () ,) ; self . sink . report_error (error) ; } }
    };
}

impl_8!();