macro_rules! deps {
    () => {
        Read!();
        FrameDecoder!();
    };
}

macro_rules! StreamingDecoder {
    () => {
        deps!();
        # [doc = " High level Zstandard frame decoder that can be used to decompress a given Zstandard frame."] # [doc = ""] # [doc = " This decoder implements `io::Read`, so you can interact with it by calling"] # [doc = " `io::Read::read_to_end` / `io::Read::read_exact` or passing this to another library / module as a source for the decoded content"] # [doc = ""] # [doc = " If you need more control over how decompression takes place, you can use"] # [doc = " the lower level [FrameDecoder], which allows for greater control over how"] # [doc = " decompression takes place but the implementor must call"] # [doc = " [FrameDecoder::decode_blocks] repeatedly to decode the entire frame."] # [doc = ""] # [doc = " ## Caveat"] # [doc = " [StreamingDecoder] expects the underlying stream to only contain a single frame,"] # [doc = " yet the specification states that a single archive may contain multiple frames."] # [doc = ""] # [doc = " To decode all the frames in a finite stream, the calling code needs to recreate"] # [doc = " the instance of the decoder and handle"] # [doc = " [crate::decoding::errors::ReadFrameHeaderError::SkipFrame]"] # [doc = " errors by skipping forward the `length` amount of bytes, see <https://github.com/KillingSpark/zstd-rs/issues/57>"] # [doc = ""] # [doc = " ```no_run"] # [doc = " // `read_to_end` is not implemented by the no_std implementation."] # [doc = " #[cfg(feature = \"std\")]"] # [doc = " {"] # [doc = "     use std::fs::File;"] # [doc = "     use std::io::Read;"] # [doc = "     use ruzstd::decoding::StreamingDecoder;"] # [doc = ""] # [doc = "     // Read a Zstandard archive from the filesystem then decompress it into a vec."] # [doc = "     let mut f: File = todo!(\"Read a .zstd archive from somewhere\");"] # [doc = "     let mut decoder = StreamingDecoder::new(f).unwrap();"] # [doc = "     let mut result = Vec::new();"] # [doc = "     Read::read_to_end(&mut decoder, &mut result).unwrap();"] # [doc = " }"] # [doc = " ```"] pub struct StreamingDecoder < READ : Read , DEC : BorrowMut < FrameDecoder > > { pub decoder : DEC , source : READ , }
    };
}

StreamingDecoder!();