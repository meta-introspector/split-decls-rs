macro_rules! deps {
    () => {
        ReadFrameHeaderError!();
        FrameDescriptorError!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl From < FrameDescriptorError > for ReadFrameHeaderError { fn from (error : FrameDescriptorError) -> Self { Self :: InvalidFrameDescriptor (error) } }
    };
}

impl_35!()