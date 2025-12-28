macro_rules! deps {
    () => {
        FrameDescriptorError!();
        FrameHeaderError!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl From < FrameDescriptorError > for FrameHeaderError { fn from (error : FrameDescriptorError) -> Self { Self :: FrameDescriptorError (error) } }
    };
}

impl_31!();