macro_rules! SerializerError {
    () => {
        # [derive (Debug)] pub enum SerializerError { Custom (String) , }
    };
}

SerializerError!()