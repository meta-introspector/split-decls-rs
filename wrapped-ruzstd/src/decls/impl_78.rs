macro_rules! deps {
    () => {
        HuffmanTableError!();
        GetBitsError!();
        DecompressLiteralsError!();
        HuffmanDecoderError!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl core :: fmt :: Display for DecompressLiteralsError { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { match self { DecompressLiteralsError :: MissingCompressedSize => { write ! (f , "compressed size was none even though it must be set to something for compressed literals" ,) } DecompressLiteralsError :: MissingNumStreams => { write ! (f , "num_streams was none even though it must be set to something (1 or 4) for compressed literals" ,) } DecompressLiteralsError :: GetBitsError (e) => write ! (f , "{e:?}") , DecompressLiteralsError :: HuffmanTableError (e) => write ! (f , "{e:?}") , DecompressLiteralsError :: HuffmanDecoderError (e) => write ! (f , "{e:?}") , DecompressLiteralsError :: UninitializedHuffmanTable => { write ! (f , "Tried to reuse huffman table but it was never initialized" ,) } DecompressLiteralsError :: MissingBytesForJumpHeader { got } => { write ! (f , "Need 6 bytes to decode jump header, got {got} bytes" ,) } DecompressLiteralsError :: MissingBytesForLiterals { got , needed } => { write ! (f , "Need at least {needed} bytes to decode literals. Have: {got} bytes" ,) } DecompressLiteralsError :: ExtraPadding { skipped_bits } => { write ! (f , "Padding at the end of the sequence_section was more than a byte long: {skipped_bits} bits. Probably caused by data corruption" ,) } DecompressLiteralsError :: BitstreamReadMismatch { read_til , expected } => { write ! (f , "Bitstream was read till: {read_til}, should have been: {expected}" ,) } DecompressLiteralsError :: DecodedLiteralCountMismatch { decoded , expected } => { write ! (f , "Did not decode enough literals: {decoded}, Should have been: {expected}" ,) } } } }
    };
}

impl_78!()