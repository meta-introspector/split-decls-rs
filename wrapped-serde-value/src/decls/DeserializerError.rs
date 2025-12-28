macro_rules! deps {
    () => {
        Unexpected!();
    };
}

macro_rules! DeserializerError {
    () => {
        deps!();
        # [derive (Debug)] pub enum DeserializerError { Custom (String) , InvalidType (Unexpected , String) , InvalidValue (Unexpected , String) , InvalidLength (usize , String) , UnknownVariant (String , & 'static [& 'static str]) , UnknownField (String , & 'static [& 'static str]) , MissingField (& 'static str) , DuplicateField (& 'static str) , }
    };
}

DeserializerError!();