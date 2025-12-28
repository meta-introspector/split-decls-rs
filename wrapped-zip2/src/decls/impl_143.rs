macro_rules! deps {
    () => {
        DateTimeRangeError!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl Error for DateTimeRangeError { }
    };
}

impl_143!();