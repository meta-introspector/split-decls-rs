macro_rules! deps {
    () => {
        DecodeBufferError!();
    };
}

macro_rules! ExecuteSequencesError {
    () => {
        deps!();
        # [derive (Debug)] # [non_exhaustive] pub enum ExecuteSequencesError { DecodebufferError (DecodeBufferError) , NotEnoughBytesForSequence { wanted : usize , have : usize } , ZeroOffset , }
    };
}

ExecuteSequencesError!()