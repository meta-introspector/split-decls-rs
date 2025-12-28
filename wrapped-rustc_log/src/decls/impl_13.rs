macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl From < SetGlobalDefaultError > for Error { fn from (tracing_error : SetGlobalDefaultError) -> Self { Error :: AlreadyInit (tracing_error) } }
    };
}

impl_13!()