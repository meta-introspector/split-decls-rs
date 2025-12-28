macro_rules! deps {
    () => {
        ReadFrameHeaderError!();
        DecodeBlockContentError!();
        DictionaryDecodeError!();
        BlockHeaderReadError!();
        FrameHeaderError!();
        Error!();
    };
}

macro_rules! FrameDecoderError {
    () => {
        deps!();
        # [derive (Debug)] # [non_exhaustive] pub enum FrameDecoderError { ReadFrameHeaderError (ReadFrameHeaderError) , FrameHeaderError (FrameHeaderError) , WindowSizeTooBig { requested : u64 } , DictionaryDecodeError (DictionaryDecodeError) , FailedToReadBlockHeader (BlockHeaderReadError) , FailedToReadBlockBody (DecodeBlockContentError) , FailedToReadChecksum (Error) , NotYetInitialized , FailedToInitialize (FrameHeaderError) , FailedToDrainDecodebuffer (Error) , FailedToSkipFrame , TargetTooSmall , DictNotProvided { dict_id : u32 } , }
    };
}

FrameDecoderError!();