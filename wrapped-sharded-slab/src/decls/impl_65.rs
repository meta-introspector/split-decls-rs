macro_rules! deps {
    () => {
        Clear!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl Clear for String { # [inline] fn clear (& mut self) { String :: clear (self) } }
    };
}

impl_65!()