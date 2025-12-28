macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl From < std :: io :: Error > for Error { fn from (error : std :: io :: Error) -> Self { Self :: io_error (error) } }
    };
}

impl_119!();