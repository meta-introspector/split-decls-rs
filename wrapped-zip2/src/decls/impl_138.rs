macro_rules! deps {
    () => {
        DateTimeRangeError!();
        ZipError!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl From < DateTimeRangeError > for ZipError { fn from (_ : DateTimeRangeError) -> Self { invalid ! ("Invalid date or time") } }
    };
}

impl_138!();