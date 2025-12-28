macro_rules! deps {
    () => {
        DeserializerError!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl From < de :: value :: Error > for DeserializerError { # [inline] fn from (e : de :: value :: Error) -> DeserializerError { DeserializerError (e . to_string ()) } }
    };
}

impl_4!()