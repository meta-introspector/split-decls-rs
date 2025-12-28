macro_rules! SerializerError {
    () => {
        # [doc = " This type represents errors that can occur when serializing."] # [derive (Debug)] pub struct SerializerError (String) ;
    };
}

SerializerError!()