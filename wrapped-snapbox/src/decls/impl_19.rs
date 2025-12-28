macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < 's > From < & 's str > for Error { fn from (other : & 's str) -> Self { Self :: with_string (other . to_owned ()) } }
    };
}

impl_19!()