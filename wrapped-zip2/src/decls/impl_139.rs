macro_rules! deps {
    () => {
        ZipError!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl From < FromUtf8Error > for ZipError { fn from (_ : FromUtf8Error) -> Self { invalid ! ("Invalid UTF-8") } }
    };
}

impl_139!();