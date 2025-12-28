macro_rules! deps {
    () => {
        ErrorSink!();
        ParseError!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < F > ErrorSink for F where F : FnMut (ParseError) , { fn report_error (& mut self , error : ParseError) { (self) (error) ; } }
    };
}

impl_14!();