macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_265 {
    () => {
        deps!();
        impl From < Time > for Value { fn from (d : Time) -> Self { let d : Datetime = d . into () ; d . into () } }
    };
}

impl_265!();