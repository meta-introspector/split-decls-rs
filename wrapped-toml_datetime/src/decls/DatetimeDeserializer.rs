macro_rules! deps {
    () => {
        Datetime!();
    };
}

macro_rules! DatetimeDeserializer {
    () => {
        deps!();
        # [doc = " Deserializer / format support for emitting [`Datetime`][crate::Datetime]"] pub struct DatetimeDeserializer < E > { date : Option < crate :: Datetime > , _error : core :: marker :: PhantomData < E > , }
    };
}

DatetimeDeserializer!()