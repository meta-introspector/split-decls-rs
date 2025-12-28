macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl bridge :: Error for Error { fn new (msg : String) -> Self { Self (msg) } fn from_internal < T : Debug > (err : T) -> Self { Self (format ! ("{err:?}")) } }
    };
}

impl_274!()