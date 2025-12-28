macro_rules! DeserializerError {
    () => {
        # [doc = " This type represents errors that can occur when deserializing."] # [derive (Debug)] pub struct DeserializerError (String) ;
    };
}

DeserializerError!()