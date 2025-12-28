macro_rules! deps {
    () => {
        Datetime!();
        DatetimeDeserializer!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < E > DatetimeDeserializer < E > { # [doc = " Create a deserializer to emit [`Datetime`][crate::Datetime]"] pub fn new (date : crate :: Datetime) -> Self { Self { date : Some (date) , _error : Default :: default () , } } }
    };
}

impl_43!();