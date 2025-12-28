macro_rules! DecodeBufferError {
    () => {
        # [derive (Debug)] # [non_exhaustive] pub enum DecodeBufferError { NotEnoughBytesInDictionary { got : usize , need : usize } , OffsetTooBig { offset : usize , buf_len : usize } , }
    };
}

DecodeBufferError!();