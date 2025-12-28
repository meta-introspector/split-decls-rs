macro_rules! deps {
    () => {
        GetBitsError!();
        DecodeSequenceError!();
        Read!();
        FSEDecoderError!();
        FSETableError!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl core :: fmt :: Display for DecodeSequenceError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { DecodeSequenceError :: GetBitsError (e) => write ! (f , "{e:?}") , DecodeSequenceError :: FSEDecoderError (e) => write ! (f , "{e:?}") , DecodeSequenceError :: FSETableError (e) => write ! (f , "{e:?}") , DecodeSequenceError :: ExtraPadding { skipped_bits } => { write ! (f , "Padding at the end of the sequence_section was more than a byte long: {skipped_bits} bits. Probably caused by data corruption" ,) } DecodeSequenceError :: UnsupportedOffset { offset_code } => { write ! (f , "Do not support offsets bigger than 1<<32; got: {offset_code}" ,) } DecodeSequenceError :: ZeroOffset => write ! (f , "Read an offset == 0. That is an illegal value for offsets") , DecodeSequenceError :: NotEnoughBytesForNumSequences => write ! (f , "Bytestream did not contain enough bytes to decode num_sequences") , DecodeSequenceError :: ExtraBits { bits_remaining } => write ! (f , "{bits_remaining}") , DecodeSequenceError :: MissingCompressionMode => write ! (f , "compression modes are none but they must be set to something") , DecodeSequenceError :: MissingByteForRleLlTable => { write ! (f , "Need a byte to read for RLE ll table") } DecodeSequenceError :: MissingByteForRleOfTable => { write ! (f , "Need a byte to read for RLE of table") } DecodeSequenceError :: MissingByteForRleMlTable => { write ! (f , "Need a byte to read for RLE ml table") } } } }
    };
}

impl_88!();