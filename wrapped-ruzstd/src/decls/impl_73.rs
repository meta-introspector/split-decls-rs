macro_rules! deps {
    () => {
        FrameDecoderError!();
        BlockHeaderReadError!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl From < BlockHeaderReadError > for FrameDecoderError { fn from (val : BlockHeaderReadError) -> Self { Self :: FailedToReadBlockHeader (val) } }
    };
}

impl_73!();