macro_rules! deps {
    () => {
        GetBitsError!();
    };
}

macro_rules! HuffmanDecoderError {
    () => {
        deps!();
        # [derive (Debug)] # [non_exhaustive] pub enum HuffmanDecoderError { GetBitsError (GetBitsError) , }
    };
}

HuffmanDecoderError!();