// Generated macro for test_block_header_reading (function)
macro_rules! Depcrate_teststest_block_header_reading {
() => {
// Module: crate::tests
// Provides: {"test_block_header_reading"}
// Dependencies: {}
# [test] fn test_block_header_reading () { use crate :: decoding ; use crate :: decoding :: frame ; use std :: fs ; let mut content = fs :: File :: open ("./decodecorpus_files/z000088.zst") . unwrap () ; let (_frame , _) = frame :: read_frame_header (& mut content) . unwrap () ; let mut block_dec = decoding :: block_decoder :: new () ; let block_header = block_dec . read_block_header (& mut content) . unwrap () ; let _ = block_header ; }
};
}
