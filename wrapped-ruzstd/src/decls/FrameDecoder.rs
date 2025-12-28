macro_rules! deps {
    () => {
        Write!();
        FrameDecoderState!();
        Read!();
        Dictionary!();
    };
}

macro_rules! FrameDecoder {
    () => {
        deps!();
        # [doc = " Low level Zstandard decoder that can be used to decompress frames with fine control over when and how many bytes are decoded."] # [doc = ""] # [doc = " This decoder is able to decode frames only partially and gives control"] # [doc = " over how many bytes/blocks will be decoded at a time (so you don't have to decode a 10GB file into memory all at once)."] # [doc = " It reads bytes as needed from a provided source and can be read from to collect partial results."] # [doc = ""] # [doc = " If you want to just read the whole frame with an `io::Read` without having to deal with manually calling [FrameDecoder::decode_blocks]"] # [doc = " you can use the provided [crate::decoding::StreamingDecoder] wich wraps this FrameDecoder."] # [doc = ""] # [doc = " Workflow is as follows:"] # [doc = " ```"] # [doc = " use ruzstd::decoding::BlockDecodingStrategy;"] # [doc = ""] # [doc = " # #[cfg(feature = \"std\")]"] # [doc = " use std::io::{Read, Write};"] # [doc = ""] # [doc = " // no_std environments can use the crate's own Read traits"] # [doc = " # #[cfg(not(feature = \"std\"))]"] # [doc = " use ruzstd::io::{Read, Write};"] # [doc = ""] # [doc = " fn decode_this(mut file: impl Read) {"] # [doc = "     //Create a new decoder"] # [doc = "     let mut frame_dec = ruzstd::decoding::FrameDecoder::new();"] # [doc = "     let mut result = Vec::new();"] # [doc = ""] # [doc = "     // Use reset or init to make the decoder ready to decode the frame from the io::Read"] # [doc = "     frame_dec.reset(&mut file).unwrap();"] # [doc = ""] # [doc = "     // Loop until the frame has been decoded completely"] # [doc = "     while !frame_dec.is_finished() {"] # [doc = "         // decode (roughly) batch_size many bytes"] # [doc = "         frame_dec.decode_blocks(&mut file, BlockDecodingStrategy::UptoBytes(1024)).unwrap();"] # [doc = ""] # [doc = "         // read from the decoder to collect bytes from the internal buffer"] # [doc = "         let bytes_read = frame_dec.read(result.as_mut_slice()).unwrap();"] # [doc = ""] # [doc = "         // then do something with it"] # [doc = "         do_something(&result[0..bytes_read]);"] # [doc = "     }"] # [doc = ""] # [doc = "     // handle the last chunk of data"] # [doc = "     while frame_dec.can_collect() > 0 {"] # [doc = "         let x = frame_dec.read(result.as_mut_slice()).unwrap();"] # [doc = ""] # [doc = "         do_something(&result[0..x]);"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " fn do_something(data: &[u8]) {"] # [doc = " # #[cfg(feature = \"std\")]"] # [doc = "     std::io::stdout().write_all(data).unwrap();"] # [doc = " }"] # [doc = " ```"] pub struct FrameDecoder { state : Option < FrameDecoderState > , dicts : BTreeMap < u32 , Dictionary > , }
    };
}

FrameDecoder!();