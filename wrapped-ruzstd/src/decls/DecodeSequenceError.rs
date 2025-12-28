macro_rules! deps {
    () => {
        FSETableError!();
        FSEDecoderError!();
        GetBitsError!();
    };
}

macro_rules! DecodeSequenceError {
    () => {
        deps!();
        # [derive (Debug)] # [non_exhaustive] pub enum DecodeSequenceError { GetBitsError (GetBitsError) , FSEDecoderError (FSEDecoderError) , FSETableError (FSETableError) , ExtraPadding { skipped_bits : i32 } , UnsupportedOffset { offset_code : u8 } , ZeroOffset , NotEnoughBytesForNumSequences , ExtraBits { bits_remaining : isize } , MissingCompressionMode , MissingByteForRleLlTable , MissingByteForRleOfTable , MissingByteForRleMlTable , }
    };
}

DecodeSequenceError!();