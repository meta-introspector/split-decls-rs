macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl From < & str > for Error { fn from (e : & str) -> Self { Self :: msg (e) } }
    };
}

impl_120!()