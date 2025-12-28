macro_rules! deps {
    () => {
        Datetime!();
    };
}

macro_rules! DatetimeSerializer {
    () => {
        deps!();
        # [doc = " Serializer / format support for emitting [`Datetime`][crate::Datetime]"] # [derive (Default)] pub struct DatetimeSerializer { value : Option < crate :: Datetime > , }
    };
}

DatetimeSerializer!()