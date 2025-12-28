macro_rules! deps {
    () => {
        FrameDescriptorError!();
    };
}

macro_rules! FrameHeaderError {
    () => {
        deps!();
        # [derive (Debug)] # [non_exhaustive] pub enum FrameHeaderError { WindowTooBig { got : u64 } , WindowTooSmall { got : u64 } , FrameDescriptorError (FrameDescriptorError) , DictIdTooSmall { got : usize , expected : usize } , MismatchedFrameSize { got : usize , expected : u8 } , FrameSizeIsZero , InvalidFrameSize { got : u8 } , }
    };
}

FrameHeaderError!();