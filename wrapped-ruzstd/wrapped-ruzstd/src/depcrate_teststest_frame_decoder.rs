// Generated macro for test_frame_decoder (function)
macro_rules! Depcrate_teststest_frame_decoder {
() => {
// Module: crate::tests
// Provides: {"test_frame_decoder"}
// Dependencies: {}
# [test] fn test_frame_decoder () { use crate :: decoding :: BlockDecodingStrategy ; use crate :: decoding :: FrameDecoder ; use std :: fs ; let mut content = fs :: File :: open ("./decodecorpus_files/z000088.zst") . unwrap () ; struct NullWriter (()) ; impl std :: io :: Write for NullWriter { fn write (& mut self , buf : & [u8]) -> Result < usize , std :: io :: Error > { Ok (buf . len ()) } fn flush (& mut self) -> Result < () , std :: io :: Error > { Ok (()) } } let mut _null_target = NullWriter (()) ; let mut frame_dec = FrameDecoder :: new () ; frame_dec . reset (& mut content) . unwrap () ; frame_dec . decode_blocks (& mut content , BlockDecodingStrategy :: All) . unwrap () ; }
};
}
