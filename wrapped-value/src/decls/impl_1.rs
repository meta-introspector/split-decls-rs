macro_rules! deps {
    () => {
        DeserializerError!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl de :: Error for DeserializerError { # [inline] fn custom < T : fmt :: Display > (msg : T) -> Self { DeserializerError (msg . to_string ()) } }
    };
}

impl_1!();