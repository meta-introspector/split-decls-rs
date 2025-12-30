// Generated macro for test_incremental_read (function)
macro_rules! Depcrate_teststest_incremental_read {
() => {
// Module: crate::tests
// Provides: {"test_incremental_read"}
// Dependencies: {}
# [test] fn test_incremental_read () { use crate :: decoding :: FrameDecoder ; let mut unread_compressed_content = include_bytes ! ("../../decodecorpus_files/abc.txt.zst") . as_slice () ; let mut frame_dec = FrameDecoder :: new () ; frame_dec . reset (& mut unread_compressed_content) . unwrap () ; let mut output = [0u8 ; 3] ; let (_ , written) = frame_dec . decode_from_to (unread_compressed_content , & mut output) . unwrap () ; assert_eq ! (written , 3) ; assert_eq ! (output . map (char :: from) , ['a' , 'b' , 'c']) ; assert ! (frame_dec . is_finished ()) ; let written = frame_dec . collect_to_writer (& mut & mut output [..]) . unwrap () ; assert_eq ! (written , 3) ; assert_eq ! (output . map (char :: from) , ['d' , 'e' , 'f']) ; }
};
}
