macro_rules! deps {
    () => {
        DictionaryDecodeError!();
        ReadFrameHeaderError!();
        FrameHeaderError!();
        FrameDecoderError!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl core :: fmt :: Display for FrameDecoderError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> :: core :: fmt :: Result { match self { FrameDecoderError :: ReadFrameHeaderError (e) => { write ! (f , "{e:?}") } FrameDecoderError :: FrameHeaderError (e) => { write ! (f , "{e:?}") } FrameDecoderError :: WindowSizeTooBig { requested } => { write ! (f , "Specified window_size is too big; Requested: {}, Max: {}" , requested , crate :: common :: MAX_WINDOW_SIZE ,) } FrameDecoderError :: DictionaryDecodeError (e) => { write ! (f , "{e:?}") } FrameDecoderError :: FailedToReadBlockHeader (e) => { write ! (f , "Failed to parse/decode block body: {e}") } FrameDecoderError :: FailedToReadBlockBody (e) => { write ! (f , "Failed to parse block header: {e}") } FrameDecoderError :: FailedToReadChecksum (e) => { write ! (f , "Failed to read checksum: {e}") } FrameDecoderError :: NotYetInitialized => { write ! (f , "Decoder must initialized or reset before using it" ,) } FrameDecoderError :: FailedToInitialize (e) => { write ! (f , "Decoder encountered error while initializing: {e}") } FrameDecoderError :: FailedToDrainDecodebuffer (e) => { write ! (f , "Decoder encountered error while draining the decodebuffer: {e}" ,) } FrameDecoderError :: FailedToSkipFrame => { write ! (f , "Failed to skip bytes for the length given in the frame header") } FrameDecoderError :: TargetTooSmall => { write ! (f , "Target must have at least as many bytes as the contentsize of the frame reports") } FrameDecoderError :: DictNotProvided { dict_id } => { write ! (f , "Frame header specified dictionary id 0x{dict_id:X} that wasnt provided by add_dict() or reset_with_dict()") } } } }
    };
}

impl_71!();