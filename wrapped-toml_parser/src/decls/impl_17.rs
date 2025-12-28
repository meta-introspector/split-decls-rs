macro_rules! deps {
    () => {
        ErrorSink!();
        ParseError!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [allow (unused_qualifications)] impl ErrorSink for alloc :: vec :: Vec < ParseError > { fn report_error (& mut self , error : ParseError) { self . push (error) ; } }
    };
}

impl_17!()