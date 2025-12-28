macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_275 {
    () => {
        deps!();
        impl From < & str > for Error { fn from (value : & str) -> Self { Self (value . into ()) } }
    };
}

impl_275!()