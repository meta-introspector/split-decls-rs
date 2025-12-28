macro_rules! test_frame_header_reading {
    () => {
        # [cfg (test)] # [test] fn test_frame_header_reading () { use crate :: decoding :: frame ; use std :: fs ; let mut content = fs :: File :: open ("./decodecorpus_files/z000088.zst") . unwrap () ; let (_frame , _) = frame :: read_frame_header (& mut content) . unwrap () ; }
    };
}

test_frame_header_reading!()