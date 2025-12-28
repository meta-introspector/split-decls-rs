macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl From < String > for Error { fn from (e : String) -> Self { Self :: msg (e) } }
    };
}

impl_121!()