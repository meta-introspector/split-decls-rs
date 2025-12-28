macro_rules! deps {
    () => {
        ErrorInner!();
        Error!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        impl Error { pub (crate) fn new (inner : impl core :: fmt :: Display) -> Self { Self { inner : ErrorInner :: Custom (inner . to_string ()) , } } pub (crate) fn unsupported_type (t : Option < & 'static str >) -> Self { Self { inner : ErrorInner :: UnsupportedType (t) , } } pub (crate) fn out_of_range (t : Option < & 'static str >) -> Self { Self { inner : ErrorInner :: OutOfRange (t) , } } pub (crate) fn unsupported_none () -> Self { Self { inner : ErrorInner :: UnsupportedNone , } } pub (crate) fn key_not_string () -> Self { Self { inner : ErrorInner :: KeyNotString , } } # [cfg (feature = "display")] pub (crate) fn date_invalid () -> Self { Self { inner : ErrorInner :: DateInvalid , } } }
    };
}

impl_314!()