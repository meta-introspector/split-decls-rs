macro_rules! deps {
    () => {
        DeserializerError!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl From < de :: value :: Error > for DeserializerError { fn from (e : de :: value :: Error) -> DeserializerError { DeserializerError :: Custom (e . to_string ()) } }
    };
}

impl_8!();