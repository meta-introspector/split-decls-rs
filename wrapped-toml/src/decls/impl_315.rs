macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl From < core :: fmt :: Error > for Error { fn from (_ : core :: fmt :: Error) -> Self { Self :: new ("an error occurred when writing a value") } }
    };
}

impl_315!()