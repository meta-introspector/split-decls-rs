macro_rules! deps {
    () => {
        ZipError!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl From < io :: Error > for ZipError { fn from (value : io :: Error) -> Self { Self :: Io (value) } }
    };
}

impl_137!();