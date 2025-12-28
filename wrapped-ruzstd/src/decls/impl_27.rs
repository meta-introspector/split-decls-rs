macro_rules! deps {
    () => {
        FrameDescriptorError!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl StdError for FrameDescriptorError { }
    };
}

impl_27!();