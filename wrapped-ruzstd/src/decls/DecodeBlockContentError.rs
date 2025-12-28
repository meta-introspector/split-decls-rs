macro_rules! deps {
    () => {
        BlockType!();
        Error!();
        DecompressBlockError!();
    };
}

macro_rules! DecodeBlockContentError {
    () => {
        deps!();
        # [derive (Debug)] # [non_exhaustive] pub enum DecodeBlockContentError { DecoderStateIsFailed , ExpectedHeaderOfPreviousBlock , ReadError { step : BlockType , source : Error } , DecompressBlockError (DecompressBlockError) , }
    };
}

DecodeBlockContentError!();