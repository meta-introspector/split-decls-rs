macro_rules! deps {
    () => {
        FrameDecoderError!();
        FrameHeaderError!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl From < FrameHeaderError > for FrameDecoderError { fn from (val : FrameHeaderError) -> Self { Self :: FrameHeaderError (val) } }
    };
}

impl_74!()