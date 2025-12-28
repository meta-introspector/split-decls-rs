macro_rules! deps {
    () => {
        DatetimeParseError!();
        DatetimeSerializer!();
    };
}

macro_rules! SerializerError {
    () => {
        deps!();
        # [doc = " See [`DatetimeSerializer`]"] # [derive (Debug)] # [non_exhaustive] pub enum SerializerError { # [doc = " Unsupported datetime format"] InvalidFormat (crate :: DatetimeParseError) , # [doc = " Unsupported serialization protocol"] InvalidProtocol , }
    };
}

SerializerError!()