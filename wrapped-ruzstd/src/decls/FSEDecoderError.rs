macro_rules! deps {
    () => {
        GetBitsError!();
    };
}

macro_rules! FSEDecoderError {
    () => {
        deps!();
        # [derive (Debug)] # [non_exhaustive] pub enum FSEDecoderError { GetBitsError (GetBitsError) , TableIsUninitialized , }
    };
}

FSEDecoderError!();