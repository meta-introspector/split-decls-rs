macro_rules! deps {
    () => {
        DateTimeRangeError!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl From < TryFromIntError > for DateTimeRangeError { fn from (_value : TryFromIntError) -> Self { DateTimeRangeError } }
    };
}

impl_141!()