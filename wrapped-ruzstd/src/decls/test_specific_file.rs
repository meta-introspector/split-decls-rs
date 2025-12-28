macro_rules! deps {
    () => {
        Read!();
        Error!();
        Write!();
        FrameDecoder!();
        BlockDecodingStrategy!();
    };
}

macro_rules! test_specific_file {
    () => {
        deps!();
        # [test] fn test_specific_file () { use crate :: decoding :: BlockDecodingStrategy ; use crate :: decoding :: FrameDecoder ; use std :: fs ; use std :: io :: BufReader ; use std :: io :: Read ; let path = "./decodecorpus_files/z000068.zst" ; let mut content = fs :: File :: open (path) . unwrap () ; struct NullWriter (()) ; impl std :: io :: Write for NullWriter { fn write (& mut self , buf : & [u8]) -> Result < usize , std :: io :: Error > { Ok (buf . len ()) } fn flush (& mut self) -> Result < () , std :: io :: Error > { Ok (()) } } let mut _null_target = NullWriter (()) ; let mut frame_dec = FrameDecoder :: new () ; frame_dec . reset (& mut content) . unwrap () ; frame_dec . decode_blocks (& mut content , BlockDecodingStrategy :: All) . unwrap () ; let result = frame_dec . collect () . unwrap () ; let original_f = BufReader :: new (fs :: File :: open ("./decodecorpus_files/z000088") . unwrap ()) ; let original : Vec < u8 > = original_f . bytes () . map (| x | x . unwrap ()) . collect () ; std :: println ! ("Results for file: {path}") ; if original . len () != result . len () { std :: println ! ("Result has wrong length: {}, should be: {}" , result . len () , original . len ()) ; } let mut counter = 0 ; let min = if original . len () < result . len () { original . len () } else { result . len () } ; for idx in 0 .. min { if original [idx] != result [idx] { counter += 1 ; } } if counter > 0 { std :: println ! ("Result differs in at least {counter} bytes from original") ; } }
    };
}

test_specific_file!()