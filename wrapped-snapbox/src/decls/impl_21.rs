macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl From < String > for Error { fn from (other : String) -> Self { Self :: with_string (other) } }
    };
}

impl_21!()