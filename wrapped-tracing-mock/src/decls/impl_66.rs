macro_rules! deps {
    () => {
        SetActualSpanIdError!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl error :: Error for SetActualSpanIdError { }
    };
}

impl_66!()