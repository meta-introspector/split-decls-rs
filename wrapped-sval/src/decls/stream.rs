macro_rules! deps {
    () => {
        Stream!();
        Value!();
        Result!();
    };
}

macro_rules! stream {
    () => {
        deps!();
        # [doc = "\nStream a value through a stream.\n"] pub fn stream < 'sval > (stream : & mut (impl Stream < 'sval > + ? Sized) , value : & 'sval (impl Value + ? Sized) ,) -> Result { stream . value (value) }
    };
}

stream!();