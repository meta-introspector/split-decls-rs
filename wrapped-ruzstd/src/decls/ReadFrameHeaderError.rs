macro_rules! deps {
    () => {
        Error!();
        FrameDescriptorError!();
    };
}

macro_rules! ReadFrameHeaderError {
    () => {
        deps!();
        # [derive (Debug)] # [non_exhaustive] pub enum ReadFrameHeaderError { MagicNumberReadError (Error) , BadMagicNumber (u32) , FrameDescriptorReadError (Error) , InvalidFrameDescriptor (FrameDescriptorError) , WindowDescriptorReadError (Error) , DictionaryIdReadError (Error) , FrameContentSizeReadError (Error) , SkipFrame { magic_number : u32 , length : u32 } , }
    };
}

ReadFrameHeaderError!();