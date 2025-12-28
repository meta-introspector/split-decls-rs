macro_rules! deps {
    () => {
        ValueSerializer!();
    };
}

macro_rules! impl_382 {
    () => {
        deps!();
        impl ValueSerializer { # [doc = " Creates a new serializer generate a TOML document."] pub fn new () -> Self { Self { } } }
    };
}

impl_382!()