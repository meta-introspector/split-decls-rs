macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl From < LexError > for Error { fn from (err : LexError) -> Self { Error :: new (err . span () , err) } }
    };
}

impl_188!();