macro_rules! FrameDescriptorError {
    () => {
        # [derive (Debug)] # [non_exhaustive] pub enum FrameDescriptorError { InvalidFrameContentSizeFlag { got : u8 } , }
    };
}

FrameDescriptorError!()