macro_rules! deps {
    () => {
        Value!();
        Stream!();
        Result!();
    };
}

macro_rules! stream_computed {
    () => {
        deps!();
        # [doc = "\nStream a value through a stream with an arbitrarily short lifetime.\n"] pub fn stream_computed < 'sval > (stream : & mut (impl Stream < 'sval > + ? Sized) , value : impl Value ,) -> Result { stream . value_computed (& value) }
    };
}

stream_computed!();