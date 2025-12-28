macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        impl From < Date > for Value { fn from (d : Date) -> Self { let d : Datetime = d . into () ; d . into () } }
    };
}

impl_264!()