macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < 's > From < & 's String > for Error { fn from (other : & 's String) -> Self { Self :: with_string (other . clone ()) } }
    };
}

impl_20!();