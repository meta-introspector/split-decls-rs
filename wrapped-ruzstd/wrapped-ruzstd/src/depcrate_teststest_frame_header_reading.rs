// Generated macro for test_frame_header_reading (function)
macro_rules! Depcrate_teststest_frame_header_reading {
() => {
// Module: crate::tests
// Provides: {"test_frame_header_reading"}
// Dependencies: {}
# [cfg (test)] # [test] fn test_frame_header_reading () { use crate :: decoding :: frame ; use std :: fs ; let mut content = fs :: File :: open ("./decodecorpus_files/z000088.zst") . unwrap () ; let (_frame , _) = frame :: read_frame_header (& mut content) . unwrap () ; }
};
}
