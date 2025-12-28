macro_rules! deps {
    () => {
        HuffmanScratch!();
        Sequence!();
        FSEScratch!();
        DecodeBuffer!();
    };
}

macro_rules! DecoderScratch {
    () => {
        deps!();
        # [doc = " A block level decoding buffer."] pub struct DecoderScratch { # [doc = " The decoder used for Huffman blocks."] pub huf : HuffmanScratch , # [doc = " The decoder used for FSE blocks."] pub fse : FSEScratch , pub buffer : DecodeBuffer , pub offset_hist : [u32 ; 3] , pub literals_buffer : Vec < u8 > , pub sequences : Vec < Sequence > , pub block_content_buffer : Vec < u8 > , }
    };
}

DecoderScratch!();