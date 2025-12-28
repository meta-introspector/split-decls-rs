macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_281 {
    () => {
        deps!();
        impl From < io :: Error > for Error { fn from (value : io :: Error) -> Self { Error (value . to_string ()) } }
    };
}

impl_281!();