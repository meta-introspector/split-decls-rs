macro_rules! deps {
    () => {
        DecodeBufferError!();
        ExecuteSequencesError!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl From < DecodeBufferError > for ExecuteSequencesError { fn from (val : DecodeBufferError) -> Self { Self :: DecodebufferError (val) } }
    };
}

impl_85!();