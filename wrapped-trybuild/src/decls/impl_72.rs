macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl From < GlobError > for Error { fn from (err : GlobError) -> Self { Error :: Glob (err) } }
    };
}

impl_72!()