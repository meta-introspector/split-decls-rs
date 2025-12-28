macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl From < PatternError > for Error { fn from (err : PatternError) -> Self { Error :: Pattern (err) } }
    };
}

impl_73!();