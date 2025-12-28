macro_rules! deps {
    () => {
        Error!();
        Write!();
        FrameDecoder!();
        BlockDecodingStrategy!();
    };
}

macro_rules! test_frame_decoder {
    () => {
        deps!();
        # [test] fn test_frame_decoder () { use crate :: decoding :: BlockDecodingStrategy ; use crate :: decoding :: FrameDecoder ; use std :: fs ; let mut content = fs :: File :: open ("./decodecorpus_files/z000088.zst") . unwrap () ; struct NullWriter (()) ; impl std :: io :: Write for NullWriter { fn write (& mut self , buf : & [u8]) -> Result < usize , std :: io :: Error > { Ok (buf . len ()) } fn flush (& mut self) -> Result < () , std :: io :: Error > { Ok (()) } } let mut _null_target = NullWriter (()) ; let mut frame_dec = FrameDecoder :: new () ; frame_dec . reset (& mut content) . unwrap () ; frame_dec . decode_blocks (& mut content , BlockDecodingStrategy :: All) . unwrap () ; }
    };
}

test_frame_decoder!()