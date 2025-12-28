macro_rules! deps {
    () => {
        FrameDecoderError!();
        ReadFrameHeaderError!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl From < ReadFrameHeaderError > for FrameDecoderError { fn from (val : ReadFrameHeaderError) -> Self { Self :: ReadFrameHeaderError (val) } }
    };
}

impl_75!()